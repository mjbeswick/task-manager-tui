# Spec Delta

## MODIFIED Requirements

### Requirement: Menu bar is keyboard operable

The menu bar SHALL expose File, Options, View, Process, and Help. F10 SHALL focus the
menu bar, the arrow keys SHALL move between menus and items, Enter SHALL activate an
item, and Esc SHALL close the menu without acting.

#### Scenario: Open a menu
- **WHEN** the user presses F10 and then Right twice
- **THEN** the View menu opens and its items are listed with their shortcut hints

#### Scenario: Process menu
- **WHEN** the user opens Process
- **THEN** it lists End task, End tree, Signal, Priority, and Properties

#### Scenario: Alt shortcuts
- **WHEN** the user presses Alt+P
- **THEN** the Process menu opens at its first item

#### Scenario: Activate an item
- **WHEN** an item is highlighted and the user presses Enter
- **THEN** the item's action runs, the menu closes, and focus returns to the content area

#### Scenario: Unavailable item
- **WHEN** an item cannot be used on this platform
- **THEN** it is drawn disabled with a short reason, and activating it does nothing

#### Scenario: No process selected
- **WHEN** a process action is chosen on a tab with no selected process
- **THEN** the existing notice explains the requirement and nothing is signalled
