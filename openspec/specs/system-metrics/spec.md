# system-metrics Specification

## Purpose
Collects live system and per-process measurements on a fixed cadence, converts raw
counters into the rates and percentages every tab displays, and keeps a bounded
history for the graphs.

## Requirements

### Requirement: Sampling cadence

The system SHALL sample the machine on a fixed interval, defaulting to one second,
and SHALL offer High (0.5 s), Normal (1 s), Low (2 s), and Paused. Changing the speed
MUST take effect without restarting the application.

#### Scenario: Default cadence
- **WHEN** the application starts with no saved setting
- **THEN** a new sample is produced about once per second

#### Scenario: Speed change
- **WHEN** the user selects Low in the Options menu
- **THEN** subsequent samples arrive about every two seconds and the status bar reports Low

#### Scenario: Paused
- **WHEN** the user pauses sampling
- **THEN** displayed values stop changing, keys and scrolling still respond, and resuming refreshes immediately

### Requirement: CPU percentages are whole-system percentages

CPU figures SHALL be normalised so that every core at full load reports 100%, as
Windows 10 does. A process using one full core on an eight-core machine MUST report
12.5%.

#### Scenario: Single busy core
- **WHEN** one process saturates one core of an eight-core machine
- **THEN** that process shows 12.5% and the system total shows 12.5%

#### Scenario: Full load
- **WHEN** every core is fully busy
- **THEN** the system total shows 100% and no value exceeds it

#### Scenario: Total equals parts
- **WHEN** the process rows are summed for a sample
- **THEN** the sum stays within a small tolerance of the system total for that sample

### Requirement: Rates derive from counter deltas

Disk and network throughput SHALL be computed from counter differences over the
elapsed wall time of two samples. The first sample after start or resume MUST report
zero instead of a spike.

#### Scenario: First sample
- **WHEN** the application has just started
- **THEN** every throughput figure reads zero and no graph draws a full-height spike

#### Scenario: Pause and resume
- **WHEN** sampling resumes after a pause
- **THEN** the first figure after resume is zero and the following figure reflects the real rate

#### Scenario: Counter reset
- **WHEN** a counter decreases between samples, as after a device reset
- **THEN** that figure reports zero for the sample instead of a negative or huge value

### Requirement: Process identity survives reuse

Each process row SHALL be identified by its process ID and start time together, so a
reused process ID MUST NOT inherit another process's history, CPU time, or selection.

#### Scenario: Process ID reused
- **WHEN** a process exits and the system later assigns its process ID to a new program
- **THEN** the new process starts a fresh row with no inherited history

#### Scenario: Exited selection
- **WHEN** the selected process disappears
- **THEN** the selection moves to the nearest remaining row and the application does not panic

### Requirement: Bounded history

Each graph series SHALL keep the most recent 60 samples and discard the oldest. Memory
use MUST NOT grow with run time.

#### Scenario: Long run
- **WHEN** the application has been sampling for an hour
- **THEN** each series still reports 60 samples and memory use has not grown with uptime

#### Scenario: New sample
- **WHEN** a sample arrives while a series holds 60 values
- **THEN** the oldest value is dropped, the newest is appended, and the graph scrolls by one column

### Requirement: Process status labels

The system SHALL map platform process states to the Windows 10 labels Running,
Suspended, Not responding, and Stopped. A zombie MUST read Stopped.

#### Scenario: Running process
- **WHEN** a process is executing or runnable
- **THEN** its status reads Running

#### Scenario: Zombie
- **WHEN** a process has exited but its parent has not reaped it
- **THEN** its status reads Stopped and End task is disabled

#### Scenario: Uninterruptible wait
- **WHEN** a process is blocked in kernel I/O
- **THEN** its status reads Not responding

### Requirement: Sampling failures are non-fatal

A failed sample SHALL leave the previous values in place, show a dismissible message
naming the failure, and retry on the next tick.

#### Scenario: Transient failure
- **WHEN** one sample cannot read part of the system
- **THEN** the last good values stay on screen, a message appears, and sampling continues

#### Scenario: Recovery
- **WHEN** the next sample succeeds
- **THEN** the message clears on its own and values update normally

### Requirement: Sampling never blocks the interface

Collection SHALL happen away from the key-handling path. With a sample taking several
seconds, key presses MUST still be handled promptly.

#### Scenario: Slow sample
- **WHEN** producing a sample takes two seconds
- **THEN** tab changes and menu navigation still respond within a tenth of a second and the previous data stays visible

#### Scenario: Overlapping work
- **WHEN** a sample is still running as the next one is due
- **THEN** the next one is skipped rather than queued, and no backlog builds up
