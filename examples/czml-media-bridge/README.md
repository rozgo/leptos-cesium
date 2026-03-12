# CZML Media Stream

Demonstrates automatic media assignment for a moving CZML rectangle with
explicit append updates:

1. Load base CZML using `CzmlDataSource` in `Replace` mode.
2. Let `CzmlDataSource` automatically apply flattened `properties.media_*` fields to the rectangle.
3. Send delta packets with `mode=Append` (`process()`) via trigger.
4. Keep the video material stable across geometry updates while the rectangle moves.
5. Show a static expected route overlay for the video rectangle path.

Run:

```bash
trunk serve --open
```
