# Spec Delta

## MODIFIED Requirements

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, each prefixed by its digit. The active tab MUST
be drawn as an open box with box-drawing characters: a top border, sides, and `└` `┘`
corners on the rule row, with an empty bottom.

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

#### Scenario: Active tab box
- **WHEN** a tab is active and visible
- **THEN** its label sits under a `┌─┐` top border and between two `│` sides

#### Scenario: Bottom corners
- **WHEN** the rule row is drawn under the active tab
- **THEN** a `└` closes the left side, a `┘` closes the right side, and the space between them is empty

#### Scenario: Rule continuity
- **WHEN** the rule meets the active tab
- **THEN** it stops at the `└`, resumes after the `┘`, and does not run under the label

#### Scenario: Narrow strip
- **WHEN** the terminal is narrower than 90 columns
- **THEN** labels shorten and the active box and its corners are drawn around the shortened label if it is visible

#### Scenario: Active tab hidden
- **WHEN** the strip is too narrow for the active tab
- **THEN** no box is drawn and the remaining labels keep the continuous rule
