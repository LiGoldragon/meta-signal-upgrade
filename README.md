# meta-signal-upgrade

`meta-signal-upgrade` is the meta-policy signal contract for the
`upgrade` triad.

It carries catalogue-policy authority and selector authority in the active
upgrade meta contract. The old `owner-signal-sema-upgrade` prototype is
archived; its catalogue-policy role is merged here.
`AttemptHandover` is intentionally retired; peers request upgrades through
the ordinary `signal-upgrade::AttemptUpgrade` operation, while this
meta-signal contract configures policy and emergency selector controls.
