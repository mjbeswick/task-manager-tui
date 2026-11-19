# Spec Delta

## Purpose

Presents the Windows 10 Task Manager window - menu bar, seven-tab strip, content
area, and status bar - inside a terminal, driven entirely by the keyboard.

## ADDED Requirements

### Requirement: Window layout matches Windows 10

The application SHALL draw a menu bar at the top, a tab strip directly beneath it,
the active tab's content in the middle, and a status bar at the bottom, filling the
whole terminal. It MUST use the flat Windows 10 borders rather than a double-line
box.

#### Scenario: Standard terminal
- **WHEN** the application starts in a terminal of at least 100 columns by 30 rows
- **THEN** the menu bar, all seven tab labels, the content area, and the status bar are visible without scrolling

#### Scenario: Narrow terminal
- **WHEN** the terminal is narrower than 100 columns
- **THEN** optional columns are dropped first, the process name stays visible, and the tab strip remains readable

#### Scenario: Terminal below the minimum
- **WHEN** the terminal is smaller than 40 columns by 12 rows
- **THEN** the application shows a single message naming the required minimum size and draws no partial layout

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order. The active tab MUST be visually distinct using
the Windows 10 accent underline.

#### Scenario: Cycle tabs
- **WHEN** the user presses Tab or Shift+Tab
- **THEN** the active tab moves to the next or previous tab, wrapping at the ends

#### Scenario: Jump to a tab
- **WHEN** the user presses a digit from 1 to 7
- **THEN** the matching tab becomes active

#### Scenario: Tab state is kept
- **WHEN** the user leaves a tab and returns to it
- **THEN** its selection, scroll position, and sort order are as they were left

### Requirement: Menu bar is keyboard operable

The menu bar SHALL expose File, Options, View, and Help. F10 SHALL focus the menu
bar, the arrow keys SHALL move between menus and items, Enter SHALL activate an item,
and Esc SHALL close the menu without acting.

#### Scenario: Open a menu
- **WHEN** the user presses F10 and then Right twice
- **THEN** the View menu opens and its items are listed with their shortcut hints

#### Scenario: Activate an item
- **WHEN** an item is highlighted and the user presses Enter
- **THEN** the item's action runs, the menu closes, and focus returns to the content area

#### Scenario: Unavailable item
- **WHEN** an item cannot be used on this platform
- **THEN** it is drawn disabled with a short reason, and activating it does nothing

### Requirement: Light and dark themes

The application SHALL offer the Windows 10 light theme and the Windows 10 dark theme,
SHALL start in light unless settings say otherwise, and SHALL keep the chosen theme
across restarts.

#### Scenario: Default appearance
- **WHEN** the application starts with no saved settings
- **THEN** the light theme is used, with a near-white background, dark text, and the Windows 10 blue accent

#### Scenario: Switch theme
- **WHEN** the user selects Dark mode from the Options menu
- **THEN** every visible element switches to the dark palette immediately and the choice is saved

#### Scenario: Readable contrast
- **WHEN** any theme is active
- **THEN** text and its background differ enough to read, and colour is never the only signal for selection

### Requirement: Dialog model

The application SHALL present confirmations, errors, the run dialog, and help as
modal panels drawn over the window. A modal panel MUST capture all keys until it
closes.

#### Scenario: Confirmation
- **WHEN** an action needs confirmation
- **THEN** a panel names the target and the action, offers a default and a cancel choice, and stays open until one is chosen

#### Scenario: Error
- **WHEN** an action fails
- **THEN** a panel shows the failure reason and a hint about privilege, and the underlying table stays unchanged

#### Scenario: Dismiss
- **WHEN** the user presses Esc in any dialog
- **THEN** the dialog closes and no action is taken

### Requirement: Every action is reachable from the keyboard

Every action the application offers SHALL be reachable from the keyboard, including
arrow and vim-style movement, half-page jumps, first and last row, and menu access
with Alt plus the menu's first letter. No feature MUST require a pointer.

#### Scenario: No pointer
- **WHEN** the user completes all seven tabs, every menu item, and every dialog using only the keyboard
- **THEN** no control is left unreachable

#### Scenario: Movement styles
- **WHEN** the content area is focused
- **THEN** Up/Down and k/j move one row, PgUp/PgDn and Ctrl+u/Ctrl+d move ten, g and G jump to the first and last row, and h/l collapse and expand

#### Scenario: Direct menu access
- **WHEN** the user presses Alt+F, Alt+O, Alt+V, or Alt+H
- **THEN** the File, Options, View, or Help menu opens with its first item selected

### Requirement: Mouse support

The application SHALL capture mouse input and SHALL treat clicks as shortcuts for
actions that also exist on the keyboard. A click MUST NOT be the only way to do
anything, and the terminal's mouse mode MUST be released on exit.

#### Scenario: Click a tab
- **WHEN** the user clicks a tab label
- **THEN** that tab becomes active and its previous state is restored

#### Scenario: Click a row
- **WHEN** the user clicks a table row
- **THEN** that row becomes selected, and a second click on the same row within 400 milliseconds performs the row's Enter action

#### Scenario: Click the menus
- **WHEN** the user clicks a menu label and then an item
- **THEN** the menu opens on that label and the item's action runs

#### Scenario: Click a resource
- **WHEN** the user clicks CPU, Memory, Disk, Network, or GPU in the Performance rail
- **THEN** that resource's panel is shown

#### Scenario: Wheel
- **WHEN** the user scrolls the wheel over the content area
- **THEN** the selection moves three rows in the wheel's direction

#### Scenario: Click dialog buttons
- **WHEN** the user clicks a confirm, cancel, or close button in a dialog
- **THEN** the same action runs as its key would run

#### Scenario: Mouse mode restored
- **WHEN** the application exits, including through a panic
- **THEN** mouse reporting is disabled along with the alternate screen and raw mode

### Requirement: Terminal state restored

The application SHALL restore the terminal on every exit path, including an
unexpected error, and SHALL leave no alternate screen, hidden cursor, or raw mode
behind.

#### Scenario: Normal exit
- **WHEN** the user quits with q or Ctrl+C
- **THEN** the previous screen, cursor, and input mode are restored

#### Scenario: Panic
- **WHEN** the application panics
- **THEN** the terminal is restored before the panic message is printed

### Requirement: Status bar summary

The status bar SHALL show the current CPU, memory, disk, and network totals, the
active update speed, and a hint for the Fewer details toggle, matching the Windows 10
bottom bar.

#### Scenario: Live totals
- **WHEN** a new sample arrives
- **THEN** the status bar totals update with it and keep a fixed width so the layout does not shift

#### Scenario: Unavailable metric
- **WHEN** a metric has no value on this platform
- **THEN** the status bar shows a dash for it and keeps the remaining totals in place
