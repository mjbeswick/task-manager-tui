# Design

## Context

- `platform::service_action` refuses system scope before any command runs.
- Signals use `libc::kill` in-process; EPERM is reported after the fact.
- The app captures child stdio for service commands and has no password UI.

## Decisions

### One elevated action value

`platform::ElevatedAction` is either a service command or a signal with its PIDs. The
effect layer carries that value; the main loop executes it. Nothing else in the app gains
root.

### Build argv, then quote only for AppleScript

Linux runs `pkexec <program> <args...>` directly. macOS `do shell script` needs one string,
so each argument is single-quoted, with embedded single quotes escaped. No user text is
interpolated without quoting.

### Helper order

- macOS: `osascript` with `with administrator privileges`.
- Linux: `pkexec` if present; otherwise `sudo -n`; if it reports that a password is
  required, the UI opens a masked field and retries with `sudo -S`, writing the password to
  stdin only.

### Password lifetime

The field lives in a `Dialog::Password` and is dropped when the dialog closes. It is never
written to settings, history, logs, or the screen. The elevated effect carries it only for
the call.

### Signals are pre-emptively elevated

A target owned by another user or root goes straight to the elevated path after
confirmation, rather than trying `libc::kill` and reacting to EPERM. This avoids a partial
tree delivery.

## Risks / Trade-offs

- [Headless Linux] `pkexec` without an agent fails → The password fallback covers it.
- [Password in memory] Rust strings are not zeroed → Best effort; the value is short-lived
  and never persisted.
- [macOS quoting] `do shell script` runs through `sh` → Arguments are quoted defensively and
  contain only fixed program names, service labels, signal names, and PIDs.
