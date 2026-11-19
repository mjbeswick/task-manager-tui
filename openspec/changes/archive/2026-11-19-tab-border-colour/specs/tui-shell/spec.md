# Spec Delta

## MODIFIED Requirements

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, each prefixed by its digit. The active tab MUST
be drawn as an open box whose outline uses the rule colour, with an accent label.

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

#### Scenario: No separators
- **WHEN** the tab strip is drawn
- **THEN** labels are separated by empty space and no vertical bar appears between them

#### Scenario: Matching outline
- **WHEN** a tab is active and visible
- **THEN** its top border, sides, and outward bottom corners use the same colour as the rule

#### Scenario: Accent label
- **WHEN** a tab is active and visible
- **THEN** only its label uses the accent colour

#### Scenario: Narrow strip
- **WHEN** the terminal is narrower than 90 columns
- **THEN** labels shorten and the outline and corners are still drawn around the active label if it is visible

#### Scenario: Active tab hidden
- **WHEN** the strip is too narrow for the active tab
- **THEN** no box is drawn and the remaining labels keep the continuous rule
