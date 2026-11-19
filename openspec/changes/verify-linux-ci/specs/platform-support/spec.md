## ADDED Requirements

### Requirement: Every push is verified on both platforms

Continuous integration SHALL run format, lint, test, and release build on Linux and
macOS for every push to main and every pull request. A failure on either platform
MUST fail the run.

#### Scenario: Push to main
- **WHEN** a commit is pushed to main
- **THEN** CI runs every check on Ubuntu and macOS and reports each platform separately

#### Scenario: Linux only failure
- **WHEN** a test passes on macOS but fails on Linux
- **THEN** the run fails and names the Linux job

### Requirement: Renders without a service manager

The application SHALL render every tab in a Linux container that has no systemd and
no login sessions. Tabs whose data source is missing MUST show a message and MUST
NOT exit with an error.

#### Scenario: Container smoke render
- **WHEN** `taskmgr --once` renders each tab inside a minimal Linux container
- **THEN** every run exits zero and prints a frame containing the tab strip

#### Scenario: No systemd
- **WHEN** the Services tab is rendered without systemd
- **THEN** the tab shows a message saying services are unavailable
