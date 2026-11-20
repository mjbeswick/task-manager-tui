# process-list Specification

## Purpose
Defines the Processes tab: the grouped table of applications, background processes,
and system processes, in the shape Windows 10 shows, with sorting, expansion, and
end task.

## Requirements

### Requirement: Process grouping

The Processes tab SHALL group rows under Apps, Background processes, and System
processes, in that order, and each group header MUST show the number of processes it
contains.

#### Scenario: Group order
- **WHEN** the tab is opened
- **THEN** Apps appears first, then Background processes, then System processes, each with its count

#### Scenario: Classification
- **WHEN** a process belongs to a graphical application bundle
- **THEN** it is listed under Apps

#### Scenario: System processes
- **WHEN** a process runs as root or is a kernel or session service
- **THEN** it is listed under System processes

#### Scenario: Everything else
- **WHEN** a process is neither an application nor a system process
- **THEN** it is listed under Background processes

#### Scenario: Grouped rows
- **WHEN** several processes share one application name
- **THEN** they appear as one row showing the combined CPU and memory and the number of processes, with the individual processes beneath it

### Requirement: Process columns

The tab SHALL show Name, Status, CPU, Memory, Disk, Network, and GPU in the Windows 10
order. More details MUST be the default view.

#### Scenario: Available metrics
- **WHEN** a process has measured CPU, memory, and disk activity
- **THEN** each column shows a value formatted with one decimal place for percentages and a readable unit for memory and throughput

#### Scenario: Unavailable metric
- **WHEN** a metric cannot be read on this platform for a process
- **THEN** its cell shows a dash and never a zero

#### Scenario: Column widths
- **WHEN** the terminal is resized
- **THEN** the Name column absorbs the extra space and numeric columns keep a right-aligned fixed width

### Requirement: Fewer details

The tab SHALL offer a Fewer details view that shows only Name, CPU, and Memory, and
toggling it MUST preserve the selected process.

#### Scenario: Toggle to fewer details
- **WHEN** the user selects Fewer details
- **THEN** only Name, CPU, and Memory remain, rows become taller, and the same process stays selected

#### Scenario: Toggle back
- **WHEN** the user selects More details
- **THEN** all columns return with the previous sort order and selection

### Requirement: Expansion and collapse

Each group and each grouped row SHALL expand and collapse from the keyboard, and the
tab MUST remember expansion state while the application runs.

#### Scenario: Collapse a group
- **WHEN** the selected row is a group header and the user presses Left or Enter
- **THEN** its children are hidden and the header shows a collapsed marker

#### Scenario: Expand a row
- **WHEN** the selected row is a collapsed grouped row and the user presses Right or Enter
- **THEN** its child processes appear directly beneath it

#### Scenario: Collapse all
- **WHEN** the user selects Collapse all from the View menu
- **THEN** only group headers remain visible and the first group stays selected

### Requirement: Sorting

The tab SHALL sort by any column, SHALL default to CPU descending, and MUST show a
direction marker in the sorted column header.

#### Scenario: Default sort
- **WHEN** the tab first opens
- **THEN** rows are ordered by CPU from highest to lowest

#### Scenario: Change sort column
- **WHEN** the user moves the sort column to Memory
- **THEN** rows reorder by memory and the header marks the direction

#### Scenario: Reverse direction
- **WHEN** the user reverses the current sort
- **THEN** the order flips and the marker follows

#### Scenario: Stable order
- **WHEN** two rows have equal values in the sort column
- **THEN** they keep a consistent order between samples instead of swapping

### Requirement: End task

The tab SHALL end the selected process, SHALL ask for confirmation before ending a
system process or a process owned by another user, and MUST report a refusal instead
of silently failing.

#### Scenario: End an owned process
- **WHEN** the user ends a process they own
- **THEN** the process receives a termination request and the row disappears on the next sample

#### Scenario: Confirm a system process
- **WHEN** the user ends a system process
- **THEN** a confirmation panel names the process and requires an explicit choice before signalling

#### Scenario: Permission refused
- **WHEN** the operating system refuses the signal
- **THEN** a dialog explains that the process belongs to another user or needs elevation, and the table is unchanged

#### Scenario: Already gone
- **WHEN** the selected process exits before the signal is sent
- **THEN** the dialog says the process no longer exists and the selection moves to the nearest row

#### Scenario: Force
- **WHEN** a process ignores the termination request
- **THEN** the user can choose a force option that sends an unconditional termination

### Requirement: Navigation is not interrupted by samples

While the user is moving the selection, the tab SHALL hold the current row order for
two seconds and refresh values in place. A move MUST NOT be undone by a later sample.

#### Scenario: Move during a refresh
- **WHEN** the user moves one row and a sample arrives within two seconds
- **THEN** the same row stays selected and the rows keep their positions under the cursor

#### Scenario: After the pause
- **WHEN** no navigation key has been pressed for two seconds
- **THEN** the next sample re-sorts rows by the active column

#### Scenario: No revert
- **WHEN** the user moves to a different row
- **THEN** a later sample never restores the previously selected row

#### Scenario: Values stay live
- **WHEN** the order is held during navigation
- **THEN** CPU, memory, disk, network, and GPU values in every row still update on each sample

### Requirement: Selection is stable across samples

The selected process SHALL remain selected when samples arrive and when the sort order
places it elsewhere in the list, and the view MUST scroll to keep it visible.

#### Scenario: Row moves
- **WHEN** a sample changes the sort position of the selected process
- **THEN** the selection follows that process and stays on screen

#### Scenario: Manual scroll
- **WHEN** the user scrolls away without moving the selection
- **THEN** the selection stays where it was and the header shows that the view is scrolled

### Requirement: Filter by name

The tab SHALL filter rows by a case-insensitive substring of the process name, SHALL
show the active filter, and MUST restore all rows when the filter is cleared.

#### Scenario: Filter
- **WHEN** the user types a filter string
- **THEN** only matching rows and their groups remain visible, the filter field shows the text, and the match in each name is highlighted

#### Scenario: No match
- **WHEN** the filter matches nothing
- **THEN** the table shows an empty-state message rather than a blank panel

#### Scenario: Clear
- **WHEN** the user clears the filter
- **THEN** every row returns and the previous selection is restored if it still exists

### Requirement: Process signals

The Processes tab SHALL offer a signal dialog for the selected process, or for the first
process under a selected kind heading, listing TERM, KILL, INT, HUP, STOP, and CONT, and
SHALL allow the signal to target the process tree instead of one process.

#### Scenario: Open the signal dialog
- **WHEN** a process row is selected and the user presses `K`
- **THEN** a panel names the process and its PID and lists the six signals with a tree toggle

#### Scenario: Open from a heading
- **WHEN** a kind heading such as Apps is selected and the user presses `K`
- **THEN** the dialog opens for the first process under that heading and names it

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
- **WHEN** the heading has no process and no process row is selected
- **THEN** a notice asks for a process row and no signal is sent

#### Scenario: Permission refused
- **WHEN** the operating system refuses the signal
- **THEN** a dialog explains that the process belongs to another user or needs administrator rights, and the table is unchanged

#### Scenario: Already gone
- **WHEN** the process exits before the signal is delivered
- **THEN** the dialog reports that the process no longer exists

### Requirement: System totals on the Apps row

The Processes tab SHALL show the system CPU, memory, disk, network, and GPU totals in the
corresponding numeric columns of the Apps heading row, right-aligned beneath the column
they summarise, and MUST omit them when that row is not visible.

#### Scenario: Apps visible
- **WHEN** the Apps heading is the first visible row
- **THEN** the totals appear on that row, aligned under CPU, Memory, Disk, Network, and GPU, and the heading text stays readable

#### Scenario: Scrolled away
- **WHEN** the viewport no longer includes the Apps heading
- **THEN** no totals are drawn on any process row

#### Scenario: Filtered out
- **WHEN** the filter hides the Apps heading
- **THEN** no totals are drawn

#### Scenario: Dropped columns
- **WHEN** a narrow terminal drops optional columns
- **THEN** totals are drawn only for the columns that remain

#### Scenario: Live values
- **WHEN** a new sample arrives while the Apps row is visible
- **THEN** the totals update in place without moving the heading text

#### Scenario: Header height
- **WHEN** the terminal is tall
- **THEN** the column header still uses one row and the table starts one row higher than before

### Requirement: Resource cells are shaded by load

The tab SHALL shade CPU, Memory, Disk, and Network cells by the row's share of the
system total, in five steps from a pale tint to a deep tint. A higher share MUST
never get a lighter step than a lower share. Text on every step MUST keep 4.5:1
contrast.

#### Scenario: Busy process
- **WHEN** a process uses 40% of the CPU
- **THEN** its CPU cell is drawn in a deeper step than a process using 2%

#### Scenario: Idle value
- **WHEN** a cell's share is below the first step
- **THEN** the cell keeps the plain row background

#### Scenario: Unavailable metric
- **WHEN** a cell shows a dash
- **THEN** the cell is never shaded

#### Scenario: Selected row
- **WHEN** a shaded row is selected
- **THEN** the selection background replaces the shading across the whole row

#### Scenario: Group rows
- **WHEN** a group heading row is drawn
- **THEN** its cells are not shaded

### Requirement: Header shows system totals

The column header SHALL show the system total above each resource label, as a
percentage for CPU and Memory and as a rate for Disk and Network.

#### Scenario: Totals line
- **WHEN** the tab is drawn with a sample
- **THEN** the header shows `27%` above `CPU` when total CPU is 27 percent

#### Scenario: Unknown total
- **WHEN** a total cannot be read on this platform
- **THEN** a dash is shown above that label

#### Scenario: Short terminal
- **WHEN** the content area is shorter than twelve rows
- **THEN** the totals line is dropped and the header takes one row

### Requirement: Process details modal

The Processes tab SHALL open a modal with the selected process's details when the user
presses Enter or `i` on a process row, SHALL refresh those details on every sample, and
MUST close on Enter, Esc, or q without changing the selection.

#### Scenario: Open on a process row
- **WHEN** a child process row is selected and the user presses Enter
- **THEN** a modal names the process and shows its identity, ownership, state, and current metrics

#### Scenario: Open with i
- **WHEN** a process row is selected and the user presses `i`
- **THEN** the same modal opens

#### Scenario: Group row
- **WHEN** a group or kind heading is selected and the user presses Enter
- **THEN** the group expands or collapses as before and no modal opens

#### Scenario: Live values
- **WHEN** a new sample arrives while the modal is open
- **THEN** CPU, memory, disk, network, and GPU values update in place

#### Scenario: Close
- **WHEN** the user presses Enter, Esc, or q
- **THEN** the modal closes and the selected row is unchanged

#### Scenario: Process exits
- **WHEN** the process disappears while the modal is open
- **THEN** the modal states that it has exited instead of showing stale values

#### Scenario: Unavailable metrics
- **WHEN** a platform cannot report a metric
- **THEN** the modal shows a dash for that field

#### Scenario: Command line
- **WHEN** the process has a command line
- **THEN** the modal shows it, and shows a dash when it is empty

#### Scenario: Narrow terminal
- **WHEN** the terminal is too narrow for a long command line
- **THEN** the line is truncated inside the modal rather than wrapping outside it
