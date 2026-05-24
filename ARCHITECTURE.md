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

## U1 Shape

U1 is intentionally skeletal. The channel has no domain operations yet;
it keeps the generated observability verbs and `RequestUnimplemented`
reply so the runtime placeholder has a typed failure shape.

U3 populates this crate with the merged owner surface from
`owner-signal-sema-upgrade` and `owner-signal-version-handover`:
`Register`, `Allow`, `Block`, `Query`, `ForceFlip`, `Rollback`, and
`Quarantine`. `AttemptHandover` does not land here; `AttemptUpgrade` on
the ordinary contract subsumes it in the upgrade daemon.

## Code Map

- `src/lib.rs` declares the scaffold channel and placeholder rejection
  records.
- `tests/round_trip.rs` proves the skeleton owner channel round-trips
  through NOTA and Signal frames.
- `examples/canonical.nota` records the current placeholder text shape.

## Invariants

- Owner-only operations live here because caller authority, not touched
  state, determines the contract split.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The owner and ordinary contracts remain separate repositories.
- This crate depends on `signal-upgrade`; U3 consumes its ordinary
  shared upgrade vocabulary rather than duplicating it.
