# skills - meta-signal-upgrade

Read this before editing the upgrade meta-signal contract.

## Required Context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`

## Boundary

This crate owns only the owner-only `upgrade` meta-signal vocabulary. It has
no runtime, actors, sockets, storage, migration modules, Persona handover
driver, or systemd integration.

## Invariants

- U1 stays scaffold-only. Do not move `owner-signal-sema-upgrade`,
  `owner-signal-version-handover`, `sema-upgrade`, or Persona code into
  this crate in U1.
- U3 is the first population step for the merged meta-signal contract.
- `AttemptHandover` does not return in this contract; the working
  contract's `AttemptUpgrade` is the upgrade request verb.
- `RequestUnimplemented` stays available so partial implementations can
  return typed skeleton replies.
- Round-trip tests cover both NOTA and Signal-frame encoding.
