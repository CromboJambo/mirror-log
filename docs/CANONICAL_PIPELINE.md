# Canonical Pipeline

Mirror-Log uses one canonical order of operations:

1. `capture` - collect raw input from CLI, file, or stdin.
2. `persist` - append immutable event record to `events`.
3. `structure` - derive explicit structure (currently chunking) from persisted content.
4. `enrich` - human-level interpretation performed through explicit structure.

The first three stages are automated by the system.  
`enrich` is intentionally human-driven and should use explicit structure instead of hidden heuristics.

## Nested Governance Structure

All pipeline behavior should be interpreted through this nesting:

`law -> principle -> right -> rule -> guideline`

### Law

Non-negotiable invariants.

- Events are append-only.
- Persist happens before structure.
- Derived structures must point to a persisted event.

### Principle

Design direction used to make tradeoffs.

- Sequence before scale.
- Explicit structure over implicit behavior.

### Right

Guarantees the system owes the user.

- Deterministic event ordering.
- Inspectable SQL state.
- Recoverable lineage from event to derived structure.

### Rule

Concrete operational constraints.

- Use persistence receipts for canonical timestamps.
- Structure stage must consume persisted event IDs.
- Batch ingestion preserves input order inside each batch.

### Guideline

Tunable defaults that can change without breaking laws.

- Auto-chunk threshold: `2000` bytes.
- Chunk size: `1500` bytes.
- Stdin batch size default: `1000`.

## Command Mapping

- `add` -> capture, persist, structure, enrich (human)
- `add-file` -> capture, persist, structure, enrich (human)
- `stdin` -> capture, persist, structure, enrich (human)

All three ingestion paths now share the same canonical pipeline implementation.
