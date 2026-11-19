# Spec Delta

## MODIFIED Requirements

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
- **WHEN** the user scrolls the wheel over a list
- **THEN** that list's viewport moves three rows in the wheel's direction and the selection is unchanged

#### Scenario: Click dialog buttons
- **WHEN** the user clicks a confirm, cancel, or close button in a dialog
- **THEN** the same action runs as its key would run

#### Scenario: Mouse mode restored
- **WHEN** the application exits, including through a panic
- **THEN** mouse reporting is disabled along with the alternate screen and raw mode
