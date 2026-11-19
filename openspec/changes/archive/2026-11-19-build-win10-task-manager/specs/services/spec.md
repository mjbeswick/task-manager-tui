# Spec Delta

## Purpose

Defines the Services tab: the machine's background services as its native manager
reports them, their running state, and start, stop, and restart actions.

## ADDED Requirements

### Requirement: Services come from the native manager

The tab SHALL list services from the platform's service manager and MUST NOT infer
services from process names.

#### Scenario: Linux
- **WHEN** the tab is opened on Linux
- **THEN** systemd units, both system and user scope, are listed with their unit names

#### Scenario: macOS
- **WHEN** the tab is opened on macOS
- **THEN** launchd jobs are listed with their labels for the user and system domains the application can read

#### Scenario: Manager unavailable
- **WHEN** the service manager cannot be queried
- **THEN** the tab shows the manager's message and marks the tab as unavailable rather than empty

### Requirement: Service columns

The tab SHALL show Name, PID, Description, Status, and Group in the Windows 10 order,
and PID MUST be a dash for a service that is not running.

#### Scenario: Running service
- **WHEN** a service has a live process
- **THEN** its PID is shown and its status reads Running

#### Scenario: Stopped service
- **WHEN** a service has no process
- **THEN** its PID is a dash and its status reads Stopped

#### Scenario: Missing description
- **WHEN** the manager reports no description
- **THEN** the Description cell shows a dash

#### Scenario: Group
- **WHEN** services are grouped
- **THEN** the Group column names the domain or scope the service belongs to, such as user or system

### Requirement: Start, stop, and restart

The tab SHALL offer Start, Stop, and Restart for the selected service, SHALL perform
them through the native manager, and MUST refuse an action that needs administrator
rights with an explanation.

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
- **THEN** a dialog explains that administrator rights are required and no command is run

#### Scenario: Manager rejects
- **WHEN** the manager returns a failure
- **THEN** a dialog shows the manager's message and the row keeps its previous state

### Requirement: Filter and refresh

The tab SHALL filter by service name and description, SHALL refresh with the sample
cadence, and MUST reflect changes made outside the application after a refresh.

#### Scenario: Filter
- **WHEN** the user types a filter
- **THEN** only matching services remain and the status line shows the filter

#### Scenario: External change
- **WHEN** a service is started outside the application
- **THEN** the next refresh shows it Running with its PID

#### Scenario: No match
- **WHEN** the filter matches no service
- **THEN** an empty-state message is shown instead of a blank table

### Requirement: Service actions are confirmed

Stopping or restarting a service SHALL ask for confirmation naming the service, and the
confirmation MUST state that dependent services or jobs may be affected.

#### Scenario: Confirm stop
- **WHEN** the user chooses Stop
- **THEN** a panel names the service and its dependants warning, and no action happens before a choice

#### Scenario: Cancel
- **WHEN** the user cancels
- **THEN** the service is untouched and the table stays as it was
