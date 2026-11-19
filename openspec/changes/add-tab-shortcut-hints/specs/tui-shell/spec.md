# Spec Delta

## ADDED Requirements

### Requirement: Contextual shortcut hints

Each tab SHALL reserve its last content row for one hint line listing that tab's own
shortcuts as `[key] action`, SHALL order the most used actions first, and MUST clip the
line at the right edge instead of wrapping or covering a data row.

#### Scenario: Processes hints
- **WHEN** the Processes tab is shown
- **THEN** the line reads `[e] End task`, `[K] Signal`, `[p] Priority`, and the current details toggle

#### Scenario: Details hints
- **WHEN** the Details tab is shown
- **THEN** the line names the panel toggle, end task, signal, and priority

#### Scenario: Services hints
- **WHEN** the Services tab is shown
- **THEN** the line names start, stop, and restart and keeps the scope note

#### Scenario: Startup hints
- **WHEN** a startup entry is selected
- **THEN** the line names enable or disable according to its state and shows the command after the key

#### Scenario: Users hints
- **WHEN** the Users tab is shown
- **THEN** the line names show or hide details according to the panel state

#### Scenario: App history hints
- **WHEN** the App history tab is shown
- **THEN** the line names the properties action

#### Scenario: Performance hints
- **WHEN** the Performance tab is shown
- **THEN** the line names resource selection and Appearance

#### Scenario: State follows the row
- **WHEN** the selected row or a toggle changes
- **THEN** the hint line updates on the next frame without leaving the previous wording

#### Scenario: Narrow terminal
- **WHEN** the hints are wider than the terminal
- **THEN** the line is cut at the right edge and the table keeps its full width

#### Scenario: No room
- **WHEN** the content area is shorter than four rows
- **THEN** the hint line is not drawn and no row is reserved
