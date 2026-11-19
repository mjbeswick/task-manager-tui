# Spec Delta

## ADDED Requirements

### Requirement: Process signals

The Processes tab SHALL offer a signal dialog for the selected process, listing TERM,
KILL, INT, HUP, STOP, and CONT, and SHALL allow the signal to target the process tree
instead of one process.

#### Scenario: Open the signal dialog
- **WHEN** a process row is selected and the user presses `K`
- **THEN** a panel names the process and its PID and lists the six signals with a tree toggle

#### Scenario: Send a graceful signal
- **WHEN** the user chooses TERM, INT, HUP, or CONT for a process they own and confirms
- **THEN** that signal is delivered to the process and the next sample reflects it

#### Scenario: Confirm an unconditional signal
- **WHEN** the user chooses KILL or STOP
- **THEN** a confirmation panel names the signal and the process before anything is sent

#### Scenario: Another user or a system process
- **WHEN** the selected process is a system process or belongs to another user
- **THEN** every signal passes through confirmation before it is sent, not only KILL and STOP

#### Scenario: Process tree
- **WHEN** the tree toggle is on and the signal is sent
- **THEN** the same signal reaches the process and every descendant listed in the current sample

#### Scenario: Nothing selected
- **WHEN** the selected row is a group heading with no resolvable process
- **THEN** a notice asks for a process row and no signal is sent

#### Scenario: Permission refused
- **WHEN** the operating system refuses the signal
- **THEN** a dialog explains that the process belongs to another user or needs administrator rights, and the table is unchanged

#### Scenario: Already gone
- **WHEN** the process exits before the signal is delivered
- **THEN** the dialog reports that the process no longer exists
