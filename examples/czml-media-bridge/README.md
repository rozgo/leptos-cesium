# CZML Video + Explicit `process()`

Demonstrates parity-style video material assignment for a CZML entity with
explicit append updates:

1. Load base CZML using `CzmlDataSource` in `Replace` mode.
2. Assign video texture once at initial load (Sandcastle-style runtime assignment).
3. Send delta packets with `mode=Append` (`process()`) via trigger.
4. Keep video material stable across geometry updates (no reassign each step).
5. Use manual reapply button only when needed.

Run:

```bash
trunk serve --open
```
