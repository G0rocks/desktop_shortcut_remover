# Desktop Shortcut Remover
A program that periodically removes desktop shortcuts.
It also deletes shortcuts as they appear keeping your desktop less cluttered than it would otherwise be :)

This program was made to help deal with the problem in [this issue](https://github.com/microsoft/winget-cli/issues/1545#issuecomment-3620746650).

## How to install
Install with cargo using:
`cargo install desktop_shortcut_remover`
Install with winget (only works after [this PR]([url](https://github.com/microsoft/winget-pkgs/pull/320398)) has been merged):
`winget install G0rocks.DesktopShortcutRemover`

## User manual
The program must be run once (can be manual or set to run on startup).
After that the program will delete shortcuts a few seconds after they are created and also check the public and user desktop daily at 10:00 (10 in the morning for AM/PM people) to delete all shortcuts on the desktop in case any are missed in the real time deletions.

To configure to run at startup, run `shell:startup` to open the startup folder and add a shortcut to the desktop shortcut remover (wherever you have it installed).

To shutdown the program you must open the task manager and end the task.

## Contributions welcome :)
