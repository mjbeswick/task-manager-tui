# Spec Delta

## MODIFIED Requirements

### Requirement: Window layout matches Windows 10

The application SHALL draw a menu bar on its own background at the top, one rule row
directly beneath it, the active tab's content in the middle, and a status bar at the
bottom, filling the whole terminal. It MUST use the flat Windows 10 borders rather than
a double-line box. The window chrome MUST NOT draw tab labels.

#### Scenario: Standard terminal
- **WHEN** the application starts in a terminal of at least 100 columns by 30 rows
- **THEN** the menu bar, the rule, the content area, and the status bar are visible without scrolling

#### Scenario: Narrow terminal
- **WHEN** the terminal is narrower than 100 columns
- **THEN** optional columns are dropped first, the process name stays visible, and the rule stays continuous

#### Scenario: Terminal below the minimum
- **WHEN** the terminal is smaller than 40 columns by 12 rows
- **THEN** the application shows a single message naming the required minimum size and draws no partial layout

#### Scenario: Menu strip background
- **WHEN** the menu bar is drawn
- **THEN** it fills its row with the chrome background, and the filter field keeps a surface-coloured inset

#### Scenario: Tab rule row
- **WHEN** the window is drawn
- **THEN** the row under the menu bar is one unbroken horizontal rule and carries no tab label

## REMOVED Requirements

### Requirement: Seven tabs in the Windows 10 order

**Reason**: The strip cost two of the terminal's rows, hugged the active label with no
padding, punched a gap in the content rule, and disappeared below 72 columns. Its
labels duplicated keys the keyboard already offers.

**Migration**: Use `1`-`7`, `Tab`, and `Shift+Tab` to switch tabs and `Alt+T` to open
the tab list. The content rule and every content row are unchanged.

## ADDED Requirements

### Requirement: Seven tabs are keyboard reachable

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, reachable from the keyboard. `Tab` and
`Shift+Tab` MUST cycle through the tabs and wrap at both ends, `1`-`7` MUST jump to
their tab, and `Alt+T` MUST open a dropdown that lists all seven tabs with their keys
and a mark on the active tab. The dropdown MUST anchor at the left edge below the
content rule.

#### Scenario: Cycle tabs
- **WHEN** the user presses Tab or Shift+Tab
- **THEN** the active tab moves to the next or previous tab, wrapping at the ends

#### Scenario: Jump to a tab
- **WHEN** the user presses a digit from 1 to 7
- **THEN** the matching tab becomes active

#### Scenario: Tab state is kept
- **WHEN** the user leaves a tab and returns to it
- **THEN** its selection, scroll position, and sort order are as they were left

#### Scenario: Tab list
- **WHEN** the user presses Alt+T
- **THEN** a dropdown lists all seven tabs with their `1`-`7` keys and a mark on the active tab

#### Scenario: No strip labels
- **WHEN** the window is drawn at any width
- **THEN** no tab label, active-tab box, or `Tabs:` button appears above the content
