# Tasks

## 1. One row per user

- [ ] 1.1 Merge sessions by user ID in the sampler, keeping the current session first, counting the rest, and setting Active when any session is active. Verify: a unit test merges four sessions for one user into one row with `+3`, Active status, and totals counted once.
- [ ] 1.2 Show the count in the Session column as `tty +N`. Verify: a render test shows `pts/0 +3` and one row for that user.
- [ ] 1.3 Regenerate the Users golden frames and run fmt, clippy, and tests. Verify: all pass and the diff touches only the Users tab.
- [ ] 1.4 Build on the Linux server and render the Users tab. Verify: one row for the user, with memory below physical memory.
