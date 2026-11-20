# Spec Delta

## ADDED Requirements

### Requirement: Process details modal

The Processes tab SHALL open a modal with the selected process's details when the user
presses Enter or `i` on a process row, SHALL refresh those details on every sample, and
MUST close on Enter, Esc, or q without changing the selection.

#### Scenario: Open on a process row
- **WHEN** a child process row is selected and the user presses Enter
- **THEN** a modal names the process and shows its identity, ownership, state, and current metrics

#### Scenario: Open with i
- **WHEN** a process row is selected and the user presses `i`
- **THEN** the same modal opens

#### Scenario: Group row
- **WHEN** a group or kind heading is selected and the user presses Enter
- **THEN** the group expands or collapses as before and no modal opens

#### Scenario: Live values
- **WHEN** a new sample arrives while the modal is open
- **THEN** CPU, memory, disk, network, and GPU values update in place

#### Scenario: Close
- **WHEN** the user presses Enter, Esc, or q
- **THEN** the modal closes and the selected row is unchanged

#### Scenario: Process exits
- **WHEN** the process disappears while the modal is open
- **THEN** the modal states that it has exited instead of showing stale values

#### Scenario: Unavailable metrics
- **WHEN** a platform cannot report a metric
- **THEN** the modal shows a dash for that field

#### Scenario: Command line
- **WHEN** the process has a command line
- **THEN** the modal shows it, and shows a dash when it is empty

#### Scenario: Narrow terminal
- **WHEN** the terminal is too narrow for a long command line
- **THEN** the line is truncated inside the modal rather than wrapping outside it
