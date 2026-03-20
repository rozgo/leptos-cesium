# CZML Overlay Media Animation

Demonstrates smooth full-CZML animation with overlay-based media tracking:

1. Load one fully generated CZML document through `CzmlDataSource`.
2. Parse flattened `properties.media_*` fields into overlay descriptors.
3. Sample each matching entity's `position` over CZML time and pin DOM media to that moving anchor.
4. Render one image overlay, one native video overlay, and one YouTube overlay while their point anchors animate.
5. Use Cesium's built-in timeline and animation widgets as the playback UI.

Run:

```bash
trunk serve --open
```
