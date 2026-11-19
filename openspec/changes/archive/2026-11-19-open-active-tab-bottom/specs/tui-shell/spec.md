# Spec Delta

## MODIFIED Requirements

### Requirement: Seven tabs in the Windows 10 order

The system SHALL present Processes, Performance, App history, Startup, Users,
Details, and Services, in that order, each prefixed by its digit. The active tab MUST
be drawn as an open box with box-drawing characters: a top border and sides, no bottom
border, sitting on a rule that stops at its sides.

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
- **THEN** its label sits under a `┌─┐` top border and between two `│` sides, with no bottom border

#### Scenario: Open bottom
- **WHEN** the rule is drawn beneath the active tab
- **THEN** it stops at the tab's left side, the space under the label is empty, and the rule resumes after the right side

#### Scenario: Narrow strip
- **WHEN** the terminal is narrower than 90 columns
- **THEN** labels shorten and the active box is drawn around the shortened label if it is visible

#### Scenario: Active tab hidden
- **WHEN** the strip is too narrow for the active tab
- **THEN** no box is drawn and the remaining labels keep the continuous rule
