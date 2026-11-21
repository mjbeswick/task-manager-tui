# Spec Delta

## MODIFIED Requirements

### Requirement: Every action is reachable from the keyboard

Every action the application offers SHALL be reachable from the keyboard, including
arrow and vim-style movement, half-page jumps, first and last row, F2 for Appearance,
and menu access with Alt plus the menu's first letter. No feature MUST require a
pointer.

#### Scenario: No pointer
- **WHEN** the user completes all seven tabs, every menu item, and every dialog using only the keyboard
- **THEN** no control is left unreachable

#### Scenario: Movement styles
- **WHEN** the content area is focused
- **THEN** Up/Down and k/j move one row, PgUp/PgDn and Ctrl+u/Ctrl+d move ten, g and G jump to the first and last row, and h/l collapse and expand

#### Scenario: Direct menu access
- **WHEN** the user presses Alt+F, Alt+O, Alt+V, Alt+P, or Alt+H
- **THEN** the matching menu opens with its first item selected

## REMOVED Requirements

### Requirement: Seven tabs in the Windows 10 order

**Reason**: The `Tabs: <active> ▾` button and its dropdown duplicated labels that belong
in the strip, and hid the strip at the widths where labels are useful. The menu was also
a second, parallel navigation path for the same seven destinations.

**Migration**: Use the strip. Click a label, press `1`-`7`, or cycle with `Tab` and
`Shift+Tab`. Labels shorten as the terminal narrows and stay visible at every supported
width.

## ADDED Requirements

### Requirement: Seven tabs in a visible strip

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, as a strip of labels below the menu bar. The
strip MUST keep all seven labels visible at every supported width, shortening them in
tiers rather than hiding any. The active tab MUST be drawn as an open box whose outline
uses the rule colour, with an accent label. The strip MUST NOT print digits and MUST NOT
open a list.

#### Scenario: Full labels
- **WHEN** the terminal fits the seven full labels
- **THEN** each tab shows its full label

#### Scenario: Short labels
- **WHEN** the full labels do not fit but the short ones do
- **THEN** every tab shows its short label and none is dropped

#### Scenario: Minimum width
- **WHEN** the terminal is at the 40-column minimum
- **THEN** every tab shows a three-letter label

#### Scenario: Cycle tabs
- **WHEN** the user presses Tab or Shift+Tab
- **THEN** the active tab moves to the next or previous tab, wrapping at the ends

#### Scenario: Jump to a tab
- **WHEN** the user presses a digit from 1 to 7
- **THEN** the matching tab becomes active

#### Scenario: Click a label
- **WHEN** the user clicks a tab label
- **THEN** that tab becomes active

#### Scenario: Tab state is kept
- **WHEN** the user leaves a tab and returns to it
- **THEN** its selection, scroll position, and sort order are as they were left

#### Scenario: No list
- **WHEN** the user presses Alt+T or clicks anywhere in the tab strip
- **THEN** no tab list opens; only a label click switches tabs
