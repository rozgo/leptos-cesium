use serde_json::json;

pub const STREAM_INTERVAL_MS: i32 = 1000;
const CZML_START_EPOCH: &str = "2026-01-01T00:00:00Z";
const SIM_SECONDS_PER_STEP: f64 = 20.0;
pub const MEDIA_VIDEO_RECT_ENTITY_ID: &str = "media_video_rect";
const VIDEO_URI: &str = "https://cesium.com/public/SandcastleSampleData/big-buck-bunny_trailer.mp4";

pub fn video_uri() -> &'static str {
    VIDEO_URI
}

pub fn initial_append_step() -> usize {
    1
}

fn sample_positions(step: usize) -> (f64, f64, f64, f64, f64, f64) {
    let phase = step as f64 * 0.22;

    let driver_lon = -122.4786 + phase.sin() * 0.035;
    let driver_lat = 37.8060 + (phase * 0.7).cos() * 0.012;

    let rect_center_lon = -122.4465 + (phase * 0.45).sin() * 0.012;
    let rect_center_lat = 37.8050 + (phase * 0.35).cos() * 0.006;
    let rect_half_lon = 0.0085;
    let rect_half_lat = 0.0048;

    let west = rect_center_lon - rect_half_lon;
    let east = rect_center_lon + rect_half_lon;
    let south = rect_center_lat - rect_half_lat;
    let north = rect_center_lat + rect_half_lat;

    (driver_lon, driver_lat, west, south, east, north)
}

pub fn media_demo_czml() -> String {
    let interval = "2026-01-01T00:00:00Z/2026-01-01T00:20:00Z";
    let epoch = CZML_START_EPOCH;
    let (driver_lon_0, driver_lat_0, west_0, south_0, east_0, north_0) = sample_positions(0);
    let (driver_lon_1, driver_lat_1, west_1, south_1, east_1, north_1) = sample_positions(1);

    json!([
        {
            "id": "document",
            "version": "1.0",
            "clock": {
                "interval": interval,
                "currentTime": epoch,
                "multiplier": 20,
                "range": "LOOP_STOP",
                "step": "SYSTEM_CLOCK_MULTIPLIER"
            }
        },
        {
            "id": "media_pin",
            "name": "Pin Image",
            "position": {
                "cartographicDegrees": [-122.4786, 37.8194, 25.0]
            },
            "billboard": {
                "scale": 0.3,
                "verticalOrigin": "BOTTOM"
            },
            "properties": {
                "media": {
                    "kind": "image",
                    "target": "billboard",
                    "uri": "pin.svg"
                }
            }
        },
        {
            "id": MEDIA_VIDEO_RECT_ENTITY_ID,
            "name": "Video Rectangle",
            "rectangle": {
                "coordinates": {
                    "epoch": epoch,
                    "wsenDegrees": [
                        0.0, west_0, south_0, east_0, north_0,
                        SIM_SECONDS_PER_STEP, west_1, south_1, east_1, north_1
                    ],
                    "interpolationAlgorithm": "LAGRANGE",
                    "interpolationDegree": 1,
                    "forwardExtrapolationType": "HOLD",
                    "forwardExtrapolationDuration": 0.0
                },
                "height": 8.0,
                "outline": true,
                "outlineColor": {
                    "rgba": [230, 230, 230, 180]
                }
            },
            "properties": {
                "media": {
                    "kind": "video",
                    "target": "rectangle",
                    "uri": VIDEO_URI,
                    "autoplay": true,
                    "loop": true,
                    "muted": true,
                    "cross_origin": "anonymous"
                }
            }
        },
        {
            "id": "ride_driver",
            "name": "Ride Driver",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": [
                    0.0, driver_lon_0, driver_lat_0, 20.0,
                    SIM_SECONDS_PER_STEP, driver_lon_1, driver_lat_1, 20.0
                ],
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "billboard": {
                "image": "pin.svg",
                "scale": 0.22,
                "verticalOrigin": "BOTTOM"
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": 600,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [64, 200, 255, 220]
                        }
                    }
                }
            }
        }
    ])
    .to_string()
}

pub fn build_append_packet(step: usize) -> String {
    let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
    let (driver_lon, driver_lat, west, south, east, north) = sample_positions(step);

    json!([
        { "id": "document", "version": "1.0" },
        {
            "id": "ride_driver",
            "position": {
                "epoch": CZML_START_EPOCH,
                "cartographicDegrees": [simulation_seconds, driver_lon, driver_lat, 20.0]
            }
        },
        {
            "id": MEDIA_VIDEO_RECT_ENTITY_ID,
            "rectangle": {
                "coordinates": {
                    "epoch": CZML_START_EPOCH,
                    "wsenDegrees": [simulation_seconds, west, south, east, north]
                }
            }
        }
    ])
    .to_string()
}
