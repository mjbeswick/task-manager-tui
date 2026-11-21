# Spec Delta

## ADDED Requirements

### Requirement: Signal numbers follow the platform

The system SHALL deliver and display each signal using the host platform's number for
it. It MUST NOT assume Linux numbering on other platforms.

#### Scenario: STOP on macOS
- **WHEN** the user sends STOP on macOS
- **THEN** the process receives SIGSTOP, number 17, and the dialog shows 17

#### Scenario: CONT on macOS
- **WHEN** the user sends CONT on macOS
- **THEN** the process receives SIGCONT, number 19, and the dialog shows 19

#### Scenario: Linux
- **WHEN** the user sends STOP or CONT on Linux
- **THEN** the numbers are 19 and 18
