# users Specification

## Purpose
Defines the Users tab: the interactive sessions on the machine, each user's status, and
the resources their processes are consuming.

## Requirements

### Requirement: Sessions come from the operating system

The tab SHALL list interactive sessions reported by the system, and a background
daemon MUST NOT appear as a session.

#### Scenario: Logged-in user
- **WHEN** the tab is opened and a user has a terminal, console, or graphical session
- **THEN** one row appears for that session with the user name and session type

#### Scenario: Multiple sessions
- **WHEN** the same user has two sessions
- **THEN** both rows appear and the resources of that user are attributed to the session each process belongs to where the platform allows it

#### Scenario: No sessions
- **WHEN** the system reports no interactive sessions
- **THEN** the tab shows an empty-state message instead of an empty table

### Requirement: User columns

The tab SHALL show User, Session, Status, CPU, Memory, and Disk, and MUST mark the
session running this application. The Session column MUST identify the terminal or
console so that two sessions for one user are distinguishable.

#### Scenario: Values
- **WHEN** a row is shown
- **THEN** the Session column names the tty or console, and CPU and memory are the sum of that user's processes, formatted like the Processes tab

#### Scenario: Current session
- **WHEN** the tab lists the session this application runs in
- **THEN** that row is marked as the current session

#### Scenario: Root
- **WHEN** a session belongs to the root user
- **THEN** the row shows it as a system session with its resources

### Requirement: Session status

The Status column SHALL distinguish an active session from a disconnected or idle one
using the system's own session state.

#### Scenario: Active
- **WHEN** a session has a live terminal or graphical login
- **THEN** its status reads Active

#### Scenario: Disconnected
- **WHEN** the session exists but has no attached terminal
- **THEN** its status reads Disconnected

### Requirement: Selecting a session filters processes

Selecting a session SHALL show that user's processes in a panel or by filtering the
process table, so the numbers in the row can be traced to individual processes.

#### Scenario: Open a session
- **WHEN** the user selects a session and opens it
- **THEN** that user's processes are listed with their CPU and memory

#### Scenario: Other users' processes
- **WHEN** a process cannot be inspected because it belongs to another user
- **THEN** it is listed with the readable fields it has and dashes for the rest

### Requirement: Refreshing

The tab SHALL refresh its session list with the sample cadence, and MUST add or remove
rows when users log in or out without a restart.

#### Scenario: Login
- **WHEN** a user logs in while the tab is open
- **THEN** their row appears on the next sample

#### Scenario: Logout
- **WHEN** a session ends while the tab is open
- **THEN** its row disappears and the selection moves to a remaining row
