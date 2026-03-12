# CZML Media Animation

Demonstrates smooth full-CZML animation with automatic media assignment:

1. Load one fully generated CZML document through `CzmlDataSource`.
2. Let `CzmlDataSource` automatically apply flattened `properties.media_*` fields.
3. Animate a moving video rectangle and a moving billboard pin from sampled CZML data.
4. Keep a static route overlay to ground the motion.
5. Use Cesium's built-in timeline and animation widgets as the playback UI.

Run:

```bash
trunk serve --open
```
