# Design

`Signal::number` returns `libc::SIGTERM`, `libc::SIGKILL`, `libc::SIGINT`, `libc::SIGHUP`,
`libc::SIGSTOP`, and `libc::SIGCONT`. The direct path in `platform::signal` already calls
`libc::kill` with this number. The elevated path passes signal names to `kill -s`, so it
was never affected.

Rejected: a `cfg(target_os)` table. `libc` already carries the right values per target.
