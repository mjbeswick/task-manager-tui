# Spec Delta

## MODIFIED Requirements

### Requirement: Menu bar is keyboard operable

The menu bar SHALL expose File, Options, View, Tabs, Process, and Help. F10 SHALL focus
the menu bar, the arrow keys SHALL move between menus and items, Enter SHALL activate an
item, and Esc SHALL close the menu without acting. On a narrow terminal the labels MUST use
compact spacing so no menu is truncated.

#### Scenario: Open a menu
- **WHEN** the user presses F10 and then Right twice
- **THEN** the View menu opens and its items are listed with their shortcut hints

#### Scenario: Tabs menu
- **WHEN** the user opens Tabs
- **THEN** it lists Processes, Performance, App history, Startup, Users, Details, and Services
- **AND** choosing one switches to that tab

#### Scenario: Process menu
- **WHEN** the user opens Process
- **THEN** it lists End task, End tree, Signal, Priority, and Properties

#### Scenario: Alt shortcuts
- **WHEN** the user presses Alt+T
- **THEN** the Tabs menu opens at its first item

#### Scenario: Narrow menu bar
- **WHEN** the terminal is 40 columns wide
- **THEN** all six menu labels are drawn and clickable, with compact spacing and no truncation

#### Scenario: Activate an item
- **WHEN** an item is highlighted and the user presses Enter
- **THEN** the item's action runs, the menu closes, and focus returns to the content area

#### Scenario: Unavailable item
- **WHEN** an item cannot be used on this platform
- **THEN** it is drawn disabled with a short reason, and activating it does nothing

#### Scenario: No process selected
- **WHEN** a process action is chosen on a tab with no selected process
- **THEN** the existing notice explains the requirement and nothing is signalled
