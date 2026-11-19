# Agent Instructions

## Every change is an OpenSpec change

No change starts with code. Every change to this repository begins as an OpenSpec
change, without exception.

- Create the change first: `openspec new change <kebab-name>`.
- Write `proposal.md`, the spec deltas under `specs/<capability>/spec.md`,
  `design.md`, and `tasks.md`.
- Run `openspec validate <change> --strict` before touching source.
- Implement only after the change is approved, and tick tasks as they finish.
- Commit the artifacts together with the work they describe.
- Archive the change once its tasks and checks pass.

Bug fixes, refactors, dependency bumps, documentation, and one-line tweaks all
follow the same path. If it changes the repository, it gets a change.

The only exception is OpenSpec's own bootstrap: the commit that introduced this
rule. Nothing after it is exempt.

## Planning

The OpenSpec change is the plan. Do not also create a `PLAN.md`; `tasks.md` inside the
change is the checklist that gets ticked.

## Where artifacts live

Change artifacts live in `openspec/changes/<change>/`. Do not edit
`openspec/specs/` by hand; that tree is written by `openspec archive`.

## Checks

Before marking a task done, run the checks its task line names. Before archiving,
run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, `cargo build --release`, and `openspec validate <change> --strict`.

## Reporting

If a task cannot be verified in this environment, leave it unticked and say why.
Never tick a box on the strength of the code alone when its verification asks for
a run.
