# Design

## Context

- `HELP` is a static list of key and meaning rows.
- `Dialog::Help` closes on Esc, Enter, q, and F1.
- `settings::config_path` and `History::data_path` already return optional paths.

## Decisions

### Correct the static rows

Add `i`, change the F2 meaning to include the gradient, and split Space into startup and
process-group meanings. No structural change.

### About builds its lines from the app

Add `Dialog::About`. Its draw pass builds the lines from `app` each frame, so a theme
change while it is open is reflected. Colour depth gets a short label instead of Debug
output. Paths are shown as-is or a dash when unavailable.

*Alternative.* A fixed Properties dialog. Rejected: theme and depth would be stale.

### Menu entry

Help gains an About item. It maps to a `MenuItem::About` that opens the dialog. No new key:
the dialog is a menu action, like Run new task.
