# Spec Delta

## Purpose

Lets a single privileged action run with administrator rights without running the whole
application as root or storing a password.

## ADDED Requirements

### Requirement: Per-action elevation

The application SHALL elevate only the command for the requested service action or signal,
SHALL use an operating-system prompt where one exists, and MUST NOT run the interface as
root or persist a password.

#### Scenario: System service
- **WHEN** the user stops a system-scope service
- **THEN** a confirmation explains that administrator rights are needed, and confirming runs the same service command elevated

#### Scenario: Another user's process
- **WHEN** the user signals a process owned by another user or root
- **THEN** the signal is sent through an elevated kill command naming the same signal and PIDs

#### Scenario: Process tree
- **WHEN** the tree toggle is on for a privileged signal
- **THEN** every descendant listed in the current sample is included in the elevated command

#### Scenario: macOS prompt
- **WHEN** elevation is requested on macOS
- **THEN** the system asks for administrator approval and the application never reads the password

#### Scenario: Linux helper
- **WHEN** elevation is requested on Linux and `pkexec` is available
- **THEN** the command runs through `pkexec` with an argv, not a shell string

#### Scenario: Linux fallback
- **WHEN** `pkexec` is unavailable or has no agent and `sudo -n` cannot run
- **THEN** a masked password field accepts the sudo password for that one command and clears it afterwards

#### Scenario: Cancel
- **WHEN** the confirmation or password prompt is cancelled
- **THEN** nothing is executed and the table is unchanged

#### Scenario: Failure
- **WHEN** the elevated command fails or is refused
- **THEN** an error dialog shows the platform's message and no optimistic state change is made

#### Scenario: No password storage
- **WHEN** a password has been entered
- **THEN** it is never written to settings, history, logs, or the terminal
