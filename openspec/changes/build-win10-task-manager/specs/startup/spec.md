# Spec Delta

## Purpose

Defines the Startup tab: the per-user entries that run at login, their real enabled
state, and the actions that enable or disable them through the platform's own manager.

## ADDED Requirements

### Requirement: Startup entries come from the platform

The tab SHALL list the current user's login entries from the platform's own
registration: launch agents on macOS and autostart desktop entries on Linux. A shell
profile or cron line MUST NOT be invented as an entry.

#### Scenario: macOS
- **WHEN** the tab is opened on macOS
- **THEN** user launch agents and user-scoped login items appear with their labels and programs

#### Scenario: Linux
- **WHEN** the tab is opened on Linux
- **THEN** desktop entries from the user autostart directory and the system autostart directory appear with their names and commands

#### Scenario: No entries
- **WHEN** the user has registered no startup entries
- **THEN** the tab shows an empty-state message naming the directory it checked

### Requirement: Status is read from the system

The Status column SHALL show Enabled or Disabled based on the manager's current state
each time the tab loads, never from a value this application stored.

#### Scenario: Enabled
- **WHEN** an entry is registered and not disabled by the manager
- **THEN** its status reads Enabled

#### Scenario: Disabled elsewhere
- **WHEN** an entry is disabled outside this application and the tab is reloaded
- **THEN** its status reads Disabled on the next load

### Requirement: Enable and disable

The tab SHALL let the user enable or disable an entry for the current user. An action
that would need administrator rights MUST be refused with an explanation rather than
attempted.

#### Scenario: Disable a user entry
- **WHEN** the user disables an enabled per-user entry
- **THEN** the manager records it as disabled, the status updates, and the change survives a restart

#### Scenario: Enable
- **WHEN** the user enables a disabled per-user entry
- **THEN** the manager records it as enabled, the status updates, and the change survives a restart

#### Scenario: System scope
- **WHEN** the selected entry belongs to another user or to the system
- **THEN** a dialog explains that administrator rights are required and nothing is changed

#### Scenario: Failure
- **WHEN** the manager rejects the change
- **THEN** the dialog shows the manager's message and the status stays as it was

### Requirement: Startup impact column

The tab SHALL include the Startup impact column and MUST report "Not measured" for any
entry whose impact this application has not measured.

#### Scenario: Unmeasured
- **WHEN** an entry is shown for the first time
- **THEN** its Startup impact reads Not measured

#### Scenario: No invented rating
- **WHEN** no measurement exists for an entry
- **THEN** the column never shows High, Medium, or Low

### Requirement: Entries can be inspected

The tab SHALL show the program, arguments, scope, and source file of the selected
entry in a details panel or dialog, so the user can tell what will run.

#### Scenario: Inspect
- **WHEN** the user opens properties on an entry
- **THEN** the command line, owning user, and file it was read from are shown

#### Scenario: Missing program
- **WHEN** an entry points at a program that does not exist
- **THEN** the entry is still listed, marked as missing its program, and cannot be enabled
