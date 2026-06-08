# skills - meta-signal-upgrade

Read this before editing the upgrade meta-signal contract.

## Required Context

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`

## Boundary

This crate owns only the meta-policy `upgrade` meta-signal vocabulary. It has
no runtime, actors, sockets, storage, migration modules, Persona handover
driver, or systemd integration.

## Invariants

- `schema/lib.schema` is the source of truth for the public contract.
- `src/lib.rs` re-exports the generated contract surface; do not add a
  parallel hand-written channel.
- `AttemptHandover` does not return in this contract; the working
  contract's `AttemptUpgrade` is the upgrade request verb.
- `RequestUnimplemented` stays available so partial implementations can
  return typed skeleton replies.
- Round-trip tests cover both NOTA and Signal-frame encoding.
