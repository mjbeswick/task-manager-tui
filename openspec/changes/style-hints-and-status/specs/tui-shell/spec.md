# Spec Delta

## MODIFIED Requirements

### Requirement: Contextual shortcut hints

Each tab SHALL reserve its last content row for one hint line listing that tab's own
shortcuts as `[key] action`, SHALL draw keys in the accent colour and bold against muted
actions, SHALL order the most used actions first, and MUST clip the line at the right edge
instead of wrapping or covering a data row.

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

#### Scenario: Key emphasis
- **WHEN** the hint line is drawn
- **THEN** every `[key]` including its brackets uses the accent colour and bold, and the action text stays muted

### Requirement: Status bar summary

The status bar SHALL show the current CPU, memory, disk, and network totals and the
active update speed, matching the Windows 10 bottom bar, and MUST NOT carry the Fewer
details toggle. Labels MUST use the muted colour while values use the text colour, and
the speed value MUST use the accent.

#### Scenario: Live totals
- **WHEN** a new sample arrives
- **THEN** the status bar totals update with it and keep a fixed width so the layout does not shift

#### Scenario: Unavailable metric
- **WHEN** a metric has no value on this platform
- **THEN** the status bar shows a dash for it and keeps the remaining totals in place

#### Scenario: Details toggle stays elsewhere
- **WHEN** the Processes tab is shown
- **THEN** the Fewer details action appears in the tab's hint line and on the `f` key, not in the status bar

#### Scenario: Label contrast
- **WHEN** the status bar is drawn
- **THEN** each metric label is muted and its value is brighter, without moving the fixed fields

#### Scenario: Speed accent
- **WHEN** the update speed is shown
- **THEN** its value uses the accent colour and the fixed field width is unchanged
