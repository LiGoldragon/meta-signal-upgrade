# INTENT — meta-signal-upgrade

*The meta-policy signal contract for the `upgrade` component. Defines the
typed request/reply channel the upgrade meta authority uses to manage upgrade catalogue
policy and selector authority — force-flip, rollback, and quarantine.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `meta-signal-upgrade` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `upgrade/INTENT.md`. Ordinary upgrade attempts
stay in `signal-upgrade/INTENT.md`.

## Why this repo exists

`meta-signal-upgrade` is the **meta-policy signal contract** for the `upgrade`
runtime — the policy and authority leg of the upgrade triad beside the runtime
crate `upgrade` and the ordinary contract `signal-upgrade`. The separate
meta-signal repository keeps meta-policy-sensitive policy edits isolated from
ordinary peer communication dependencies.

## The channel shape

The meta channel has seven explicit operations:

- **Catalogue policy:** `Register` (add a migration to the catalogue),
  `Allow` (permit an attempt), `Block` (forbid an attempt), `Query` (read policy).
- **Selector authority:** `ForceFlip` (override the live selector),
  `Rollback` (revert a migration), `Quarantine` (mark a version ineligible).

`AttemptHandover` deliberately does not land here: the upgrade daemon owns
orchestration, so peers call `AttemptUpgrade` on the ordinary `signal-upgrade`
contract. meta authority configures the policy that permits or blocks those
attempts and keeps emergency selector controls available. Catalogue policy
records use contract-local `ComponentName`, `MigrationIdentifier`, and
`MigrationVersion` wire nouns.

The old `owner-signal-sema-upgrade` prototype is archived. Its
catalogue-policy role is merged into this meta-signal contract rather than
renamed into a separate `meta-signal-sema-upgrade` repository.

## Constraints

- Meta-policy operations live here because caller authority, not touched state,
  determines the contract split.
- The meta-signal and ordinary contracts remain separate repositories.
- This crate carries no daemon, actor, database, or Tokio runtime code - only
  typed meta-signal records, optional NOTA projection derives, frame aliases, and
  round-trip witnesses.
- The generated schema module is emitted with the `schema-rust-next` `WireContract`
  target, so it carries wire types, short-header codecs, and `signal-frame`
  request/reply aliases only - no Nexus/SEMA runtime terms, trace/mail helpers,
  or generic plane envelopes.
- Text projection is explicit. The default dependency graph is binary-only and
  must not pull `nota-next`, `nota-codec`, or `signal-core`; `nota-text` enables
  generated NOTA derives/impls for CLI and witness builds.

## Schema-derived stack

This contract is schema-derived. `schema/lib.schema` declares the
meta-policy upgrade meta-signal source; `build.rs` deserializes it into
`SchemaSource`, validates the schema-in-Rust value through text and rkyv
round-trips, and fails the build when the generated Rust is stale. There
is no parallel hand-written channel surface. The seven meta-policy verbs
configure the policy state that the `upgrade` runtime's catalogue and
selector reducers read.

## Non-ownership

This crate does not own:

- runtime policy storage, catalogue mutation, selector state, or migration execution;
- socket binding or Persona unit control;
- daemon-internal Signal/Nexus/SEMA plane schemas (those live inside the `upgrade`
  runtime crate);
- ordinary upgrade attempts (live in `signal-upgrade`).

## See also

- `ARCHITECTURE.md` - working shape, code map, and the schema-derived stack.
- `../upgrade/INTENT.md` - daemon-side intent (catalogue, selector, migration runtime).
- `../signal-upgrade/INTENT.md` - ordinary upgrade-attempt contract.
- `primary/skills/contract-repo.md` - contract repo discipline and naming rules.
- `primary/skills/component-triad.md` - repo triad structure and authority tiers.
