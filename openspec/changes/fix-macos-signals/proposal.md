# Proposal

## Why

`Signal::number` hardcodes Linux numbers. On macOS 19 is SIGCONT and 18 is SIGTSTP, so
STOP resumes a process and CONT suspends it. The dialog also prints the Linux numbers.

## What Changes

- Take every signal number from `libc`, so each platform gets its own values.
- The signal dialog prints the platform's number.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `process-list`: signal numbers follow the host platform.

## Impact

- `src/model.rs`: `Signal::number` and its test.
