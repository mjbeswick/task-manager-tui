# Spec Delta

## MODIFIED Requirements

### Requirement: Status bar summary

The status bar SHALL show the current CPU, memory, disk, and network totals and the
active update speed, matching the Windows 10 bottom bar, and MUST NOT carry the Fewer
details toggle.

#### Scenario: Live totals
- **WHEN** a new sample arrives
- **THEN** the status bar totals update with it and keep a fixed width so the layout does not shift

#### Scenario: Unavailable metric
- **WHEN** a metric has no value on this platform
- **THEN** the status bar shows a dash for it and keeps the remaining totals in place

#### Scenario: Details toggle stays elsewhere
- **WHEN** the Processes tab is shown
- **THEN** the Fewer details action appears in the tab's hint line and on the `f` key, not in the status bar
