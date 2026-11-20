# Spec Delta

## MODIFIED Requirements

### Requirement: Start, stop, and restart

The tab SHALL offer Start, Stop, and Restart for the selected service, SHALL perform
them through the native manager, and MUST offer per-action elevation for an action that
needs administrator rights.

#### Scenario: Stop a user service
- **WHEN** the user stops a service in their own scope
- **THEN** the manager stops it, the status becomes Stopped, and the PID becomes a dash

#### Scenario: Start a user service
- **WHEN** the user starts a stopped service in their own scope
- **THEN** the manager starts it, the status becomes Running, and its PID appears

#### Scenario: Restart
- **WHEN** the user restarts a running service
- **THEN** the service is stopped and started in one action and the row ends up Running with a new PID

#### Scenario: Privilege required
- **WHEN** the selected service belongs to the system scope
- **THEN** a confirmation explains that administrator rights are required, and confirming offers elevation for that one command

#### Scenario: Manager rejects
- **WHEN** the manager returns a failure
- **THEN** a dialog shows the manager's message and the row keeps its previous state
