# Spec Delta

## MODIFIED Requirements

### Requirement: Window layout matches Windows 10

The application SHALL draw a menu bar on its own background at the top, a tab strip
directly beneath it, a rule of centred tab underlines beneath the strip, the active
tab's content in the middle, and a status bar at the bottom, filling the whole
terminal. It MUST use the flat Windows 10 borders rather than a double-line box.

#### Scenario: Standard terminal
- **WHEN** the application starts in a terminal of at least 100 columns by 30 rows
- **THEN** the menu bar, all seven tab labels, the tab rule, the content area, and the status bar are visible without scrolling

#### Scenario: Narrow terminal
- **WHEN** the terminal is narrower than 100 columns
- **THEN** optional columns are dropped first, the process name stays visible, and the tab strip and its rule remain readable

#### Scenario: Terminal below the minimum
- **WHEN** the terminal is smaller than 40 columns by 12 rows
- **THEN** the application shows a single message naming the required minimum size and draws no partial layout

#### Scenario: Menu strip background
- **WHEN** the menu bar is drawn
- **THEN** it fills its row with the chrome background, and the filter field keeps a surface-coloured inset

#### Scenario: Tab rule row
- **WHEN** the tab strip is drawn
- **THEN** one row beneath it is reserved for the tab underlines and is not used for tab labels

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, each prefixed by its digit. The active tab MUST
be visually distinct by an accent-coloured line segment centred beneath its label,
and the tab strip MUST keep the window background.

#### Scenario: Cycle tabs
- **WHEN** the user presses Tab or Shift+Tab
- **THEN** the active tab moves to the next or previous tab, wrapping at the ends

#### Scenario: Jump to a tab
- **WHEN** the user presses a digit from 1 to 7
- **THEN** the matching tab becomes active

#### Scenario: Tab state is kept
- **WHEN** the user leaves a tab and returns to it
- **THEN** its selection, scroll position, and sort order are as they were left

#### Scenario: Tab numbers
- **WHEN** the tab strip is drawn
- **THEN** each tab shows its digit from 1 to 7 followed by its label

#### Scenario: Active tab
- **WHEN** a tab is active
- **THEN** its label uses the accent colour, the segment beneath it uses the accent colour, and every other tab keeps the window background with a border-coloured segment

#### Scenario: Centred underlines
- **WHEN** the tab strip is drawn
- **THEN** each tab has a line segment centred beneath its label with at least one empty column between neighbouring segments

#### Scenario: Narrow strip
- **WHEN** the terminal is narrower than 90 columns
- **THEN** labels shorten but every visible tab keeps its centred underline

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

#### Scenario: Click away closes a menu
- **WHEN** a menu is open and the user clicks outside its label and dropdown
- **THEN** the menu closes and the click does not select a row, switch a tab, sort a column, or start a filter

#### Scenario: Click inside the dropdown
- **WHEN** a menu is open and the user clicks its border or empty space
- **THEN** the menu stays open and nothing underneath is activated

#### Scenario: Click a resource
- **WHEN** the user clicks CPU, Memory, Disk, Network, or GPU in the Performance rail
- **THEN** that resource's panel is shown

#### Scenario: Wheel
- **WHEN** the user scrolls the wheel over the content area
- **THEN** the viewport under the pointer moves three rows in the wheel's direction and the selection does not change

#### Scenario: Click dialog buttons
- **WHEN** the user clicks a confirm, cancel, or close button in a dialog
- **THEN** the same action runs as its key would run

#### Scenario: Mouse mode restored
- **WHEN** the application exits, including through a panic
- **THEN** mouse reporting is disabled along with the alternate screen and raw mode
