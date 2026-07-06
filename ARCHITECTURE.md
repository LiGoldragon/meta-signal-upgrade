# ARCHITECTURE

## Role

`meta-signal-upgrade` owns the meta-policy signal wire vocabulary for the
`upgrade` runtime. It is the policy and authority contract leg of the
`upgrade` triad beside the runtime crate `upgrade` and the ordinary
contract `signal-upgrade`. The separate meta-signal repository keeps
meta-policy-sensitive policy edits isolated from ordinary peer
communication dependencies.

## Boundaries

This crate owns only typed meta-signal records, optional NOTA projection
derives, generated `signal-frame` aliases/codecs, and round-trip
witnesses. It does not own runtime policy storage, catalogue mutation,
selector state, migration execution, socket binding, or Persona unit
control. Daemon-internal Signal/Nexus/SEMA plane schemas live inside the
`upgrade` runtime crate, not in this external contract repository.

## Working Shape

The meta channel has seven explicit operations:

- Catalogue policy: `Register`, `Allow`, `Block`, and `Query`.
- Selector authority: `ForceFlip`, `Rollback`, and `Quarantine`.

`AttemptHandover` does not land here. The upgrade daemon owns orchestration,
so peers call `AttemptUpgrade` on the ordinary `signal-upgrade` contract.
meta authority configures the policy that permits or blocks those attempts
and keeps emergency selector controls available.

The old `owner-signal-sema-upgrade` prototype is archived. Its catalogue-policy
role is merged here rather than preserved as a separate meta-signal repo.

## Code Map

- `schema/lib.schema` declares the TrueSchema source for the meta-policy
  upgrade meta-signal surface and its generated wire-only Input/Output roots.
- `src/schema/lib.rs` is the checked-in generated Rust interface;
  `build.rs` deserializes `schema/lib.schema` through the TrueSchema build
  driver, validates source text and rkyv round-trips, and fails the build when
  the generated Rust is stale.
- `src/lib.rs` re-exports the generated schema module as the crate's
  public contract API.
- `tests/round_trip.rs` proves the merged meta channel round-trips through
  Signal frames in default mode and through NOTA under `nota-text`.
- `tests/dependency_boundary.rs` pins the feature boundary: default builds
  do not pull `nota`, `nota-codec`, or `signal-core`; `nota-text` is the
  explicit text-codec opt-in.
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
- The generated schema module is emitted with `schema-rust` in wire-contract
  mode, so it carries wire types/codecs only.
- NOTA parsing/rendering is feature-gated under `nota-text`; the default
  contract graph is binary-only for daemon consumers.
- The meta-signal and ordinary contracts remain separate repositories.
- Catalogue policy records use contract-local `ComponentName`,
  `MigrationIdentifier`, and `MigrationVersion` wire nouns.

## Schema-derived contract

**Status:** migrated. The crate's public API is emitted from
`schema/lib.schema`; there is no parallel hand-written channel surface.

`schema-rust` emits the wire types, short-header projection, request/reply
frame aliases, and binary codecs. It does not emit daemon runtime planes here.

**Per-component concerns:**
- Merged meta-signal contract per /318: catalogue policy (`Register`,
  `Allow`, `Block`, `Query`) plus selector authority (`ForceFlip`,
  `Rollback`, `Quarantine`).
- `AttemptHandover` deliberately did not land here per /318 design:
  peers call `AttemptUpgrade` on the ordinary `signal-upgrade`
  contract; meta authority configures the gating policy.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
