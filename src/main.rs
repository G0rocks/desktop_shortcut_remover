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

// Uses tokio main for async runtime
#[tokio::main]
async fn main() {
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