use serde_json::json;

const CZML_START_EPOCH: &str = "2026-01-01T00:00:00Z";
const CZML_INTERVAL: &str = "2026-01-01T00:00:00Z/2026-01-01T00:04:00Z";
const CLOCK_MULTIPLIER: i32 = 20;
const SIM_SECONDS_PER_STEP: f64 = 1.0;
const DEMO_INTERVAL_SECONDS: usize = 240;
pub const MEDIA_VIDEO_ENTITY_ID: &str = "media_video_overlay";
pub const MEDIA_YOUTUBE_ENTITY_ID: &str = "media_youtube_overlay";
const MEDIA_VIDEO_ROUTE_ENTITY_ID: &str = "media_video_expected_route";
const MEDIA_YOUTUBE_ROUTE_ENTITY_ID: &str = "media_youtube_expected_route";
const VIDEO_URI: &str = "https://cesium.com/public/SandcastleSampleData/big-buck-bunny_trailer.mp4";
const YOUTUBE_VIDEO_ID: &str = "M7lc1UVf-VE";

fn total_steps() -> usize {
    DEMO_INTERVAL_SECONDS
}

fn sample_positions(step: usize) -> (f64, f64, f64, f64) {
    let progress = step as f64 / total_steps() as f64;
    let angle = progress * std::f64::consts::TAU;

    let video_lon = -122.4786 + angle.sin() * 0.028 + (angle * 2.0).sin() * 0.004;
    let video_lat = 37.8060 + angle.cos() * 0.010 + (angle * 3.0).cos() * 0.002;

    let youtube_lon = -122.4320 + angle.cos() * 0.016 + (angle * 2.4).sin() * 0.003;
    let youtube_lat = 37.7905 + angle.sin() * 0.012 + (angle * 1.7).cos() * 0.0025;

    (video_lon, video_lat, youtube_lon, youtube_lat)
}

fn video_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let (video_lon, video_lat, _, _) = sample_positions(step);
        samples.extend([simulation_seconds, video_lon, video_lat, 20.0]);
    }

    samples
}

fn youtube_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let (_, _, youtube_lon, youtube_lat) = sample_positions(step);
        samples.extend([simulation_seconds, youtube_lon, youtube_lat, 36.0]);
    }

    samples
}

fn expected_video_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let (video_lon, video_lat, _, _) = sample_positions(step);
        positions.push(video_lon);
        positions.push(video_lat);
        positions.push(20.0);
    }

    positions
}

fn expected_youtube_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let (_, _, youtube_lon, youtube_lat) = sample_positions(step);
        positions.push(youtube_lon);
        positions.push(youtube_lat);
        positions.push(36.0);
    }

    positions
}

pub fn media_demo_czml() -> String {
    let epoch = CZML_START_EPOCH;
    let video_positions = video_samples();
    let youtube_positions = youtube_samples();
    let video_route = expected_video_route_positions();
    let youtube_route = expected_youtube_route_positions();

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
            "id": MEDIA_VIDEO_ENTITY_ID,
            "name": "Native Video Overlay",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": video_positions,
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "point": {
                "pixelSize": 14,
                "color": {
                    "rgba": [255, 176, 59, 255]
                },
                "outlineColor": {
                    "rgba": [15, 23, 42, 255]
                },
                "outlineWidth": 2
            },
            "properties": {
                "media_kind": "video",
                "media_target": "billboard",
                "media_uri": VIDEO_URI,
                "media_width": 320,
                "media_height": 180,
                "media_autoplay": true,
                "media_loop": true,
                "media_muted": true,
                "media_controls": false
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": DEMO_INTERVAL_SECONDS as f64,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [255, 176, 59, 190]
                        }
                    }
                }
            },
        },
        {
            "id": MEDIA_YOUTUBE_ENTITY_ID,
            "name": "YouTube Overlay",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": youtube_positions,
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "point": {
                "pixelSize": 14,
                "color": {
                    "rgba": [255, 77, 77, 255]
                },
                "outlineColor": {
                    "rgba": [15, 23, 42, 255]
                },
                "outlineWidth": 2
            },
            "properties": {
                "media_kind": "youtube",
                "media_target": "billboard",
                "media_youtube_id": YOUTUBE_VIDEO_ID,
                "media_width": 360,
                "media_height": 203,
                "media_autoplay": true,
                "media_muted": true,
                "media_controls": true,
                "media_start_seconds": 30
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": DEMO_INTERVAL_SECONDS as f64,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [255, 77, 77, 190]
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
        },
        {
            "id": MEDIA_YOUTUBE_ROUTE_ENTITY_ID,
            "name": "YouTube Expected Route",
            "polyline": {
                "positions": {
                    "cartographicDegrees": youtube_route
                },
                "width": 2,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [255, 77, 77, 140]
                        }
                    }
                }
            }
        }
    ])
    .to_string()
}
