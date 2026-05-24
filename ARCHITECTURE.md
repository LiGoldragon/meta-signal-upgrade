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

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** this crate's hand-written `signal_channel!` invocation + typed owner records (`Register`, `Allow`, `Block`, `Query`, `ForceFlip`, `Rollback`, `Quarantine`) converts to a single `owner-signal-upgrade/owner-signal-upgrade.schema` file consumed by the brilliant macro library (`primary-ezqx.1`). The macro emits owner-channel wire types, dispatcher, and storage descriptors for owner-policy state held by the upgrade runtime.

**Sequence:** Spirit pilots `primary-ezqx.1` first; this owner contract's schema cutover lands tightly coupled with `signal-upgrade`'s and the `upgrade` runtime's, because the seven owner verbs configure the policy state that the runtime's catalogue + selector reducers read. The cutover happens as part of the upgrade-triad-as-schema-host work named in the `upgrade` runtime's ARCH.

**Per-component concerns:**
- Merged owner contract per /318 — catalogue policy (`Register`, `Allow`, `Block`, `Query`) + selector authority (`ForceFlip`, `Rollback`, `Quarantine`).
- `AttemptHandover` deliberately did not land here per /318 design (peers call `AttemptUpgrade` on the ordinary `signal-upgrade` contract; owner authority configures the gating policy). The schema cutover preserves this owner/ordinary split — the macro emits two dispatchers, not one merged surface.
- Depends on `signal-upgrade` for `ComponentName`, `MigrationIdentifier`, migration `Version`; the schema imports that vocabulary from `signal-upgrade`'s schema.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
