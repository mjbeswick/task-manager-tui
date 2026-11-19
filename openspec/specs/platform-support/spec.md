# platform-support Specification

## Purpose
Defines what Linux and macOS each supply, how missing data and privilege failures are
surfaced, and the build and test obligations for both targets.

## Requirements

### Requirement: Two supported platforms

The application SHALL build and run on Linux and macOS as first-class targets. Every
tab MUST render on both, and the build MUST be verified on both before a release.

#### Scenario: Linux build
- **WHEN** the project is built and tested on Linux
- **THEN** every tab renders and every unit and snapshot test passes

#### Scenario: macOS build
- **WHEN** the project is built and tested on macOS
- **THEN** every tab renders and every unit and snapshot test passes

#### Scenario: Windows
- **WHEN** the project is built on Windows
- **THEN** it is not a supported target and the build is not required to succeed

### Requirement: Missing data is shown as missing

Any metric the platform does not expose SHALL render as a dash with a visible note of
why. The application MUST NOT render a missing value as zero.

#### Scenario: Unavailable metric
- **WHEN** a figure cannot be read
- **THEN** the cell shows a dash and no graph is drawn from it

#### Scenario: Zero is real
- **WHEN** a figure reads zero because the system genuinely reports zero
- **THEN** the cell shows a formatted zero such as 0.0% and not a dash

#### Scenario: Explanation
- **WHEN** the user selects a dash in a column whose values are all unavailable
- **THEN** a short explanation names the platform limitation

### Requirement: Privileges are reported, never escalated silently

The application SHALL run as the invoking user, MUST NOT request elevation itself, and
MUST tell the user which actions require administrator rights and why they failed.

#### Scenario: Refused action
- **WHEN** the operating system refuses an action for lack of privilege
- **THEN** a dialog states that administrator rights are required and suggests running the command with the platform's elevation tool

#### Scenario: No self-elevation
- **WHEN** any action needs more rights
- **THEN** the application does not relaunch itself with elevated privileges or prompt for a password

### Requirement: External commands run without a shell

Commands that query or control platform services SHALL be invoked directly with an
argument vector, and MUST NOT be passed through a shell or interpolated into a command
string.

#### Scenario: Service control
- **WHEN** a service action runs
- **THEN** it is executed as a fixed program with separate arguments and no shell metacharacter expansion

#### Scenario: Hostile name
- **WHEN** a service, unit, or label contains spaces, quotes, or shell metacharacters
- **THEN** it is passed as one argument and cannot execute additional commands

### Requirement: Platform capabilities are declared

The application SHALL expose, per platform, whether per-process network, per-process
GPU, processor affinity, handle counts, compressed memory, and link speed are
available, and the interface MUST follow those declarations.

#### Scenario: Declaration drives the interface
- **WHEN** a capability is declared unavailable on the running platform
- **THEN** every affected column, action, and stat box shows its unavailable form and is not offered as functional

#### Scenario: Capability appears
- **WHEN** a platform begins reporting a previously unavailable metric
- **THEN** the interface shows it without a code change to the tabs

### Requirement: Startup fails gracefully

The application SHALL produce a clear message and a non-zero exit when it cannot use
the terminal or read the essentials it needs, instead of leaving the terminal broken.

#### Scenario: Not a terminal
- **WHEN** the application is started with output redirected to a file
- **THEN** it exits with a message saying a terminal is required and a non-zero status

#### Scenario: Tiny terminal
- **WHEN** the terminal is smaller than the supported minimum
- **THEN** it shows the minimum size message and restores the terminal on exit

#### Scenario: Missing platform command
- **WHEN** a required manager such as systemctl or launchctl is missing
- **THEN** only the affected tab reports it as unavailable and the rest of the application works

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
