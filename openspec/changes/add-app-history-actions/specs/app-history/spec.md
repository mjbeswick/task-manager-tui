# Spec Delta

## ADDED Requirements

### Requirement: Per-entry app history actions

The App history tab SHALL let the user clear one app's accumulated totals and end that
app's running processes, each behind a confirmation that names the app.

#### Scenario: Clear one entry
- **WHEN** an entry is selected and the user presses `x` and confirms
- **THEN** that app's CPU time, peak memory, and disk totals are removed and every other entry is unchanged

#### Scenario: Cancel a clear
- **WHEN** the confirmation is cancelled
- **THEN** history is unchanged

#### Scenario: End an app's processes
- **WHEN** an entry with running processes is selected and the user presses `e` and confirms
- **THEN** every listed process for that app receives SIGTERM, and the confirmation names the count

#### Scenario: No running processes
- **WHEN** the app has no running process
- **THEN** a notice says so and no dialog is shown

#### Scenario: Permission refused
- **WHEN** one of the signals is refused
- **THEN** the existing error dialog explains the PID that failed and the rest of the table is unchanged

#### Scenario: Persisted clear
- **WHEN** the application restarts after clearing an entry
- **THEN** the cleared entry stays gone and the others remain
