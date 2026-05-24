# ARCHITECTURE

## Role

`owner-signal-upgrade` owns the owner-only wire vocabulary for the
`upgrade` runtime. It is the policy and authority contract leg of the
`upgrade` triad beside the runtime crate `upgrade` and the ordinary
contract `signal-upgrade`.

## Boundaries

This crate owns only typed owner Signal records, NOTA projection derives,
frame aliases emitted by `signal_channel!`, and round-trip witnesses. It
does not own runtime policy storage, catalogue mutation, selector state,
migration execution, socket binding, or Persona unit control.

## Working Shape

The owner channel has seven explicit operations:

- Catalogue policy: `Register`, `Allow`, `Block`, and `Query`.
- Selector authority: `ForceFlip`, `Rollback`, and `Quarantine`.

`AttemptHandover` does not land here. The upgrade daemon owns orchestration,
so peers call `AttemptUpgrade` on the ordinary `signal-upgrade` contract.
Owner authority configures the policy that permits or blocks those attempts
and keeps emergency selector controls available.

## Code Map

- `src/lib.rs` declares the merged owner channel and typed policy records.
- `tests/round_trip.rs` proves the merged owner channel round-trips through
  NOTA and Signal frames.
- `examples/canonical.nota` records stable owner text examples.

## Invariants

- Owner-only operations live here because caller authority, not touched
  state, determines the contract split.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The owner and ordinary contracts remain separate repositories.
- This crate depends on `signal-upgrade`; catalogue policy records reuse
  its `ComponentName`, `MigrationIdentifier`, and migration `Version`.
