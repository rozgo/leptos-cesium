use serde_json::json;

const CZML_START_EPOCH: &str = "2026-01-01T00:00:00Z";
const CZML_INTERVAL: &str = "2026-01-01T00:00:00Z/2026-01-01T00:04:00Z";
const CLOCK_MULTIPLIER: i32 = 20;
const SIM_SECONDS_PER_STEP: f64 = 1.0;
const DEMO_INTERVAL_SECONDS: usize = 240;
pub const MEDIA_VIDEO_RECT_ENTITY_ID: &str = "media_video_rect";
const MEDIA_VIDEO_ROUTE_ENTITY_ID: &str = "media_video_expected_route";
const VIDEO_URI: &str = "https://cesium.com/public/SandcastleSampleData/big-buck-bunny_trailer.mp4";

fn total_steps() -> usize {
    DEMO_INTERVAL_SECONDS
}

fn sample_positions(step: usize) -> (f64, f64, f64, f64, f64, f64) {
    let progress = step as f64 / total_steps() as f64;
    let angle = progress * std::f64::consts::TAU;

    let driver_lon = -122.4786 + angle.sin() * 0.028 + (angle * 2.0).sin() * 0.004;
    let driver_lat = 37.8060 + angle.cos() * 0.010 + (angle * 3.0).cos() * 0.002;

    let rect_center_lon = -122.4465 + angle.sin() * 0.010 + (angle * 2.0).sin() * 0.0025;
    let rect_center_lat = 37.8050 + angle.cos() * 0.005 + (angle * 3.0).cos() * 0.0015;
    let rect_half_lon = 0.0085;
    let rect_half_lat = 0.0048;

    let west = rect_center_lon - rect_half_lon;
    let east = rect_center_lon + rect_half_lon;
    let south = rect_center_lat - rect_half_lat;
    let north = rect_center_lat + rect_half_lat;

    (driver_lon, driver_lat, west, south, east, north)
}

fn driver_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let (driver_lon, driver_lat, _, _, _, _) = sample_positions(step);
        samples.extend([simulation_seconds, driver_lon, driver_lat, 20.0]);
    }

    samples
}

fn rectangle_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 5);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let (_, _, west, south, east, north) = sample_positions(step);
        samples.extend([simulation_seconds, west, south, east, north]);
    }

    samples
}

fn expected_video_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let (_, _, west, south, east, north) = sample_positions(step);
        positions.push((west + east) * 0.5);
        positions.push((south + north) * 0.5);
        positions.push(11.0);
    }

    positions
}

pub fn media_demo_czml() -> String {
    let epoch = CZML_START_EPOCH;
    let driver_positions = driver_samples();
    let rectangle_coordinates = rectangle_samples();
    let video_route = expected_video_route_positions();

    json!([
        {
            "id": "document",
            "version": "1.0",
            "clock": {
                "interval": CZML_INTERVAL,
                "currentTime": epoch,
                "multiplier": CLOCK_MULTIPLIER,
                "range": "LOOP_STOP",
                "step": "SYSTEM_CLOCK_MULTIPLIER"
            }
        },
        {
            "id": MEDIA_VIDEO_RECT_ENTITY_ID,
            "name": "Video Rectangle",
            "rectangle": {
                "coordinates": {
                    "epoch": epoch,
                    "wsenDegrees": rectangle_coordinates,
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
                "media_kind": "video",
                "media_target": "rectangle",
                "media_uri": VIDEO_URI,
                "media_autoplay": true,
                "media_loop": true,
                "media_muted": true,
                "media_cross_origin": "anonymous"
            }
        },
        {
            "id": "ride_driver",
            "name": "Ride Driver",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": driver_positions,
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "billboard": {
                "scale": 0.22,
                "verticalOrigin": "BOTTOM"
            },
            "properties": {
                "media_kind": "image",
                "media_target": "billboard",
                "media_uri": "pin.svg"
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": DEMO_INTERVAL_SECONDS as f64,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [64, 200, 255, 220]
                        }
                    }
                }
            }
        },
        {
            "id": MEDIA_VIDEO_ROUTE_ENTITY_ID,
            "name": "Video Expected Route",
            "polyline": {
                "positions": {
                    "cartographicDegrees": video_route
                },
                "width": 2,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [255, 176, 59, 170]
                        }
                    }
                }
            }
        }
    ])
    .to_string()
}
