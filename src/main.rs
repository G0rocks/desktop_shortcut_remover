/*
Program: desktop_shortcut_remover
Author: G0rocks
Date: 2025-12-07
Description: A program that periodically removes desktop shortcuts
*/

// Set Windows subsystem to windows to prevent console window from appearing
#![windows_subsystem = "windows"]

// Import standard library for making the Publisc desktop path
use std::path::PathBuf;

// Use tasklet for scheduling tasks
use tasklet::task::TaskStepStatusOk::Success;
use tasklet::{TaskBuilder, TaskScheduler};

// Use notify for getting events when files change. Also import necessary things from standard library
use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::{
    collections::HashMap,
    sync::mpsc,
    time::{Duration, Instant},
};



// Uses tokio main for async runtime
#[tokio::main]
async fn main() {
    // Get public desktop directory
    let public_desktop_path = PathBuf::from("C:\\Users\\Public\\Desktop");
    // Get users desktop directory
    let user_desktop_path = dirs::desktop_dir().expect("Could not find desktop directory");

    // Start file watchers on both desktop paths
    let _ = std::thread::spawn(move || {
        let _ = start_desktop_watcher(public_desktop_path);
    });
    let _ = std::thread::spawn(move || {
        let _ = start_desktop_watcher(user_desktop_path);
    });

    // Task scheduler with 24 hour loop frequency.
    let mut scheduler = TaskScheduler::default(chrono::Local);
 
    // Create tasks and add to scheduler.
    let _ = scheduler.add_task(
        TaskBuilder::new(chrono::Local)
            // 7 field cron expression for when to repeat
            .every("0 00 10 * * * *")
            // Get all shortcuts on the desktop
            .description("Remove desktop shortcuts")
            .add_step("Step 1", || {
                // Init list of shortcuts to delete
                let mut shortcuts_to_delete: Vec<String> = Vec::new();
                // Get shortcuts
                match get_desktop_shortcuts(&mut shortcuts_to_delete) {
                    // If it worked, do nothing and keep going
                    Ok(_) => (),
                    // If it failed, empty shortcuts_to_delete (in case a non-shortcut was added) then try again next time
                    Err(_) => shortcuts_to_delete.clear(),
                };
                // Delete each shortcut found
                match delete_shortcuts(shortcuts_to_delete) {
                    // If it worked, do nothing and keep going
                    Ok(_) => (),
                    // If it failed, just try again next time
                    Err(_) => (),
                };

                // Indicate that this step was a success.
                Ok(Success)
            })
            .build(),
    );

    // Execute the scheduler.
    scheduler.run().await;
}


// Helper functions
//-----------------------------------------------------------------
/// Function that gets all desktop shortcuts
/// Adds them to the provided vector
fn get_desktop_shortcuts(shortcuts: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Get public desktop directory
    let public_desktop_path = PathBuf::from("C:\\Users\\Public\\Desktop");
    // Get users desktop directory
    let user_desktop_path = dirs::desktop_dir().ok_or("Could not find desktop directory")?;

    // Iterate through all files in the public desktop directory
    for entry in std::fs::read_dir(public_desktop_path)? {
        let entry = entry?;
        let path = entry.path();
        // If file is shortcut, add to vector
        if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
            if let Some(path_str) = path.to_str() {
                shortcuts.push(path_str.to_string());
            }
        }
    }

    // Iterate through all files in the users desktop directory
    for entry in std::fs::read_dir(user_desktop_path)? {
        let entry = entry?;
        let path = entry.path();
        // If file is shortcut, add to vector
        if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
            if let Some(path_str) = path.to_str() {
                shortcuts.push(path_str.to_string());
            }
        }
    }
    
    // Return success
    Ok(())
}

/// Function that deletes files given a vector of file paths
fn delete_shortcuts(shortcut_paths: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // For each shortcut, delete the entry
    for shortcut in shortcut_paths {
        // Delete file at path
        std::fs::remove_file(shortcut)?;
    }

    // Return success
    return Ok(());
}

/// Function that starts a file watcher on the desktop path
/// For event based deletion of files
fn start_desktop_watcher(desktop_path: PathBuf) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        tx,
        notify::Config::default(),
    )?;

    watcher.watch(&desktop_path, RecursiveMode::NonRecursive)?;

    debounce_loop(rx);

    Ok(())
}

/// Function that debounces events from the file watcher to ensure the shortcut is only deleted
/// after a some time has passed since the last modification
/// This prevents deleting files that are still being modified.
fn debounce_loop(rx: mpsc::Receiver<notify::Result<notify::Event>>) {
    const GRACE: Duration = Duration::from_secs(3);

    let mut pending: HashMap<PathBuf, Instant> = HashMap::new();

    loop {
        // Collect events (non-blocking-ish)
        while let Ok(Ok(event)) = rx.try_recv() {
            if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                continue;
            }

            for path in event.paths {
                if path.extension().and_then(|e| e.to_str()) == Some("lnk") {
                    pending.insert(path, Instant::now());
                }
            }
        }

        // Check which ones have settled
        let now = Instant::now();
        let ready: Vec<PathBuf> = pending
            .iter()
            .filter(|(_, t)| now.duration_since(**t) >= GRACE)
            .map(|(p, _)| p.clone())
            .collect();

        for path in ready {
            pending.remove(&path);
            evaluate_and_maybe_delete(&path);
        }

        std::thread::sleep(Duration::from_millis(500));
    }
}

/// Function that checks if the shortcut still exists and if so, deletes it
fn evaluate_and_maybe_delete(path: &PathBuf) {
    if !path.exists() {
        return;
    }

    let _ = std::fs::remove_file(path);
}
