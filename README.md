# Desktop Shortcut Remover
A program that periodically removes desktop shortcuts

This program was made to help deal with the problem in [this issue](https://github.com/microsoft/winget-cli/issues/1545#issuecomment-3620746650).

## How to install
Install with cargo using:
`cargo install desktop_shortcut_remover`
Install with winget (only works after [this PR]([url](https://github.com/microsoft/winget-pkgs/pull/320398)) has been merged):
`winget install G0rocks.DesktopShortcutRemover

## User manual
The program must be run once (can be manual or set to run on startup).
After that the program will check the public and user desktop daily at 10:00 (10 in the morning for AM/PM people) and delete all shortcuts on the desktop.
To shutdown the program you must open the task manager and end the task.

## Contributions welcome :)
