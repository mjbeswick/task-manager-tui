# Spec Delta

## Purpose

Defines the Details tab: the flat, sortable list of every process with its process ID,
owner, and resource use, plus the per-process actions only this tab exposes.

## ADDED Requirements

### Requirement: Flat process table

The tab SHALL list every process as its own row with no grouping, in the Windows 10
column order Name, PID, Status, User name, CPU, Memory, Disk, Network, and GPU.

#### Scenario: Every process
- **WHEN** the tab is opened
- **THEN** the row count matches the processes the system reports, including background and system processes

#### Scenario: Columns
- **WHEN** a row is shown
- **THEN** PID is shown as an integer, user name as the owning account, and unavailable metrics as a dash

#### Scenario: Default sort
- **WHEN** the tab first opens
- **THEN** rows are ordered by CPU from highest to lowest

### Requirement: Sorting and selection

The tab SHALL sort by any column and MUST keep the selected process selected when
samples arrive and when the sort reorders rows.

#### Scenario: Sort by PID
- **WHEN** the user sorts by PID
- **THEN** rows order numerically by process ID and the header marks the direction

#### Scenario: Sort by user
- **WHEN** the user sorts by user name
- **THEN** rows group by account name in a stable order and respect direction

#### Scenario: Selection follows
- **WHEN** the selected process moves because of a new sample
- **THEN** the highlight stays on that process and scrolls into view

### Requirement: End process and end process tree

The tab SHALL offer end process and end process tree, SHALL confirm before ending a
process the user does not own, and MUST report the operating system's refusal.

#### Scenario: End process
- **WHEN** the user ends the selected process
- **THEN** a termination request is sent and the row disappears on the next sample

#### Scenario: End tree
- **WHEN** the user ends the process tree
- **THEN** the selected process and its descendants are signalled and the dialog names how many processes are affected

#### Scenario: Refused
- **WHEN** the operating system refuses because of ownership or privilege
- **THEN** a dialog states the reason and no rows are removed optimistically

#### Scenario: Race
- **WHEN** the process exits before the signal is delivered
- **THEN** the dialog reports that it was already gone and the selection settles on a surviving row

### Requirement: Priority

The tab SHALL show and change process priority mapped to the Windows 10 levels Low,
Below normal, Normal, Above normal, and High, and MUST translate them to the platform's
scheduling value on each target.

#### Scenario: Change priority
- **WHEN** the user sets High priority on a process they own
- **THEN** the scheduling value changes, the row shows the new priority, and the change survives the next sample

#### Scenario: Reset
- **WHEN** the user sets Normal priority
- **THEN** the process returns to the default scheduling value

#### Scenario: Not permitted
- **WHEN** the priority cannot be changed for the selected process
- **THEN** a dialog explains the permission requirement and the value is unchanged

### Requirement: Affinity is unavailable

The tab SHALL offer the Set affinity action but MUST show it disabled with a short
reason on both Linux and macOS, because neither exposes a supported per-process
affinity interface this application uses.

#### Scenario: Affinity disabled
- **WHEN** the user opens the context actions for any process
- **THEN** Set affinity appears disabled with the reason "not supported on this platform"

#### Scenario: Activation refused
- **WHEN** the user activates the disabled action
- **THEN** nothing changes and the reason stays visible

### Requirement: Process details panel

The tab SHALL show, for the selected process, its command line, executable path, parent
process, start time, thread count, and owner.

#### Scenario: Inspect
- **WHEN** the user opens details for a process
- **THEN** the listed fields are shown, with a dash where a field cannot be read

#### Scenario: Refresh
- **WHEN** a new sample arrives while the panel is open
- **THEN** the panel updates for the same process, and closes with a message if that process exits
