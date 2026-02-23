# CZML Streaming Example

This example demonstrates append-mode CZML streaming with one persistent `CzmlDataSource`, using a San Francisco ride route (Golden Gate Bridge to Fisherman's Wharf).

## What it shows

- One live CZML data source in append mode for driver deltas
- Repeated updates via `trigger` ticks
- Simulated live packets on an interval (`Start`/`Stop`)
- Manual packet sends (`Step`, `Burst x10`)
- Recenter (`ViewerZoomToTarget`) and home reset (`CameraFlyHome`)
- Stable error/loading callbacks for UI state
- Dynamic telemetry in CZML labels (speed, ETA, heading, ride number)
- Static route preview + pickup/dropoff markers + live driver telemetry
- Driver trail is built from CZML sampled-position deltas (`process` append path), not full polyline redraws

## Run

From this directory:

```bash
trunk serve
```

Then open http://localhost:8080

## Notes

- `Start` begins a timer that emits small CZML delta packets.
- Static geometry (route + markers) is loaded once; streaming packets append new position samples to the driver entity.
- The ride loops when reaching the final waypoint, incrementing ride number.
- `Burst x10` schedules ten quick updates to stress append ordering.
