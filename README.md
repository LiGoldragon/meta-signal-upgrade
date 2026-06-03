# meta-signal-upgrade

`meta-signal-upgrade` is the owner-only meta-signal contract for the
`upgrade` triad.

It merges the catalogue-policy authority from `owner-signal-sema-upgrade`
with selector authority from `owner-signal-version-handover`.
`AttemptHandover` is intentionally retired; peers request upgrades through
the ordinary `signal-upgrade::AttemptUpgrade` operation, while this
meta-signal contract configures policy and emergency selector controls.
