# app-history Specification

## Purpose
Defines the App history tab and the on-disk usage record that lets it report CPU time,
peak memory, and disk traffic per application across application restarts.

## Requirements

### Requirement: Usage is accumulated across runs

The system SHALL accumulate, per application name, total CPU time, peak memory, disk
bytes read and written, and the time it was last seen, and MUST add to that record
while the application is not limited to the current session.

#### Scenario: First run
- **WHEN** the application starts with no history file
- **THEN** the tab lists only the applications currently running, with totals counted from this session

#### Scenario: Restart
- **WHEN** the application is closed and opened again
- **THEN** totals for applications seen before continue from their stored values rather than restarting at zero

#### Scenario: Application not running
- **WHEN** a recorded application is not currently running
- **THEN** it still appears in the tab with its stored totals and a last-seen time

### Requirement: History columns

The tab SHALL show Name, CPU time, Peak memory, Disk, Last seen, and Network, in the
Windows 10 arrangement, and Network MUST show a dash when per-application network is
unavailable.

#### Scenario: Values
- **WHEN** a row is shown
- **THEN** CPU time is formatted as hours, minutes, and seconds, memory uses a readable unit, and disk shows read and written totals

#### Scenario: No network data
- **WHEN** the platform cannot attribute network traffic to applications
- **THEN** the Network column shows a dash for every row and a note explains why

#### Scenario: Sort
- **WHEN** the user sorts by CPU time
- **THEN** rows order by accumulated CPU time from highest to lowest

### Requirement: Persistence is durable and versioned

The record SHALL be written as JSON with a version field at the platform's application
data directory, SHALL be flushed periodically and on exit, and a corrupt or
incompatible file MUST NOT prevent startup.

#### Scenario: Corrupt file
- **WHEN** the history file cannot be parsed
- **THEN** it is moved aside, a message is shown, and the application starts with an empty history

#### Scenario: Interrupted write
- **WHEN** the application is killed while writing
- **THEN** the previous readable history is still available on the next start

#### Scenario: Unwritable location
- **WHEN** the data directory cannot be written
- **THEN** the tab still works for the session and a message says history will not persist

### Requirement: History can be cleared

The tab SHALL offer a delete-usage-history action that asks for confirmation, SHALL
clear stored totals for every application, and MUST keep the applications currently
running with totals reset.

#### Scenario: Confirm reset
- **WHEN** the user confirms deleting usage history
- **THEN** stored totals are erased, the table reloads, and running applications restart their totals from now

#### Scenario: Cancel
- **WHEN** the user cancels the confirmation
- **THEN** no stored data changes

### Requirement: Retention

Records not seen for 30 days SHALL be dropped when history is loaded, so the file does
not grow without limit.

#### Scenario: Old record
- **WHEN** an application was last seen more than 30 days ago
- **THEN** it is absent from the tab and its record is removed on the next save

#### Scenario: Recent record
- **WHEN** an application was seen within 30 days
- **THEN** it is kept and shown
