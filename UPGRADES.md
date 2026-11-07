# Upgrades

## 4.1.0 — repin to the 0.33 cohort

Schema and archive are unchanged; the minor version marks the text and API adoption.

- Pins: datom-codec eabead4 (0.33.0), protos 3c7ade3 (0.33.0), build-dependency ethos-zero
  5a5daca (19.2.0). One revision of each in the lock.
- Archive: every canonical example in `examples/canonical.datom` (12 Query and Response values)
  archives to the same bytes as 4.0.0 (f7b3e8d); `tests/archive_stability.rs` holds the 4.0.0 bytes.
- Text: every canonical example reads to the same value; the 0.33 printer may break a long
  line across lines, a layout difference only.
- Rust API: `src/generated/signal.rs` is regenerated with ethos-zero 19.2.0, so every scalar
  alias is a tuple struct. A consumer wraps a value with the constructor and reads it as `.0`;
  `Response::RequestUnimplemented` carries `UnimplementedRequest(UnimplementedReason::NotBuiltYet)`.
- `ReaderBudget` fields are private: build one with `ReaderBudget::try_from(n).unwrap()`.
