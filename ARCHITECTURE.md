# ARCHITECTURE

## Role

`meta-signal-upgrade` owns the meta-policy signal wire vocabulary for the
`upgrade` runtime. It is the policy and authority contract leg of the
`upgrade` triad beside the runtime crate `upgrade` and the ordinary
contract `signal-upgrade`. The separate meta-signal repository keeps
meta-policy-sensitive policy edits isolated from ordinary peer
communication dependencies.

## Boundaries

This crate owns only typed meta-signal records, NOTA projection derives,
frame aliases emitted by `signal_channel!`, and round-trip witnesses. It
does not own runtime policy storage, catalogue mutation, selector state,
migration execution, socket binding, or Persona unit control.
Daemon-internal Signal/Nexus/SEMA plane schemas live inside the
`upgrade` runtime crate, not in this external contract repository.

## Working Shape

The meta channel has seven explicit operations:

- Catalogue policy: `Register`, `Allow`, `Block`, and `Query`.
- Selector authority: `ForceFlip`, `Rollback`, and `Quarantine`.

`AttemptHandover` does not land here. The upgrade daemon owns orchestration,
so peers call `AttemptUpgrade` on the ordinary `signal-upgrade` contract.
meta authority configures the policy that permits or blocks those attempts
and keeps emergency selector controls available.

## Code Map

- `schema/lib.schema` declares the first real schema-next source for
  the meta-policy upgrade meta-signal surface and its generated
  wire-only Input/Output roots.
- `src/schema/lib.rs` is the checked-in generated Rust interface;
  `build.rs` deserializes `schema/lib.schema` into `SchemaSource`,
  validates the schema-in-Rust value through text and rkyv round-trips,
  and fails the build when the generated Rust is stale.
- `src/lib.rs` declares the merged meta channel and typed policy records.
- `tests/round_trip.rs` proves the merged meta channel round-trips through
  NOTA and Signal frames.
- `tests/generated_schema.rs` exercises generated Input/Output
  short-header/frame round-trips and guards against generated
  Nexus/SEMA runtime terms, trace/mail helpers, and generic plane
  envelopes in this contract.
- `examples/canonical.nota` records stable meta-signal text examples.

## Invariants

- Meta-policy operations live here because caller authority, not touched
  state, determines the contract split.
- The contract crate carries no daemon, actor, database, or Tokio
  runtime code.
- The generated schema module is emitted with the schema-rust
  `WireContract` target, so it carries wire types/codecs only.
- The meta-signal and ordinary contracts remain separate repositories.
- This crate depends on `signal-upgrade`; catalogue policy records reuse
  its `ComponentName`, `MigrationIdentifier`, and migration `Version`.

## Pending schema-engine upgrade

**Status:** migration started. The crate now carries checked-in
schema-next artifacts beside the hand-written `signal_channel!`
surface. The generated module is a witness surface until the runtime
cutover replaces the hand-written meta-signal contract path.

**Target:** this crate's hand-written `signal_channel!` invocation + typed meta-signal records (`Register`, `Allow`, `Block`, `Query`, `ForceFlip`, `Rollback`, `Quarantine`) converts to a single `meta-signal-upgrade/meta-signal-upgrade.schema` file consumed by the brilliant macro library (`primary-ezqx.1`). The macro emits meta-channel wire types, dispatcher, and storage descriptors for meta-policy state held by the upgrade runtime.

**Sequence:** Spirit pilots `primary-ezqx.1` first; this meta-signal contract's schema cutover lands tightly coupled with `signal-upgrade`'s and the `upgrade` runtime's, because the seven meta-policy verbs configure the policy state that the runtime's catalogue + selector reducers read. The cutover happens as part of the upgrade-triad-as-schema-host work named in the `upgrade` runtime's ARCH.

**Per-component concerns:**
- Merged meta-signal contract per /318 — catalogue policy (`Register`, `Allow`, `Block`, `Query`) + selector authority (`ForceFlip`, `Rollback`, `Quarantine`).
- `AttemptHandover` deliberately did not land here per /318 design (peers call `AttemptUpgrade` on the ordinary `signal-upgrade` contract; meta authority configures the gating policy). The schema cutover preserves this meta/ordinary split — the macro emits two dispatchers, not one merged surface.
- Depends on `signal-upgrade` for `ComponentName`, `MigrationIdentifier`, migration `Version`; the schema imports that vocabulary from `signal-upgrade`'s schema.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
