use serde_json::json;

const CZML_START_EPOCH: &str = "2026-01-01T00:00:00Z";
const CZML_INTERVAL: &str = "2026-01-01T00:00:00Z/2026-01-01T00:04:00Z";
const CLOCK_MULTIPLIER: i32 = 20;
const SIM_SECONDS_PER_STEP: f64 = 1.0;
const DEMO_INTERVAL_SECONDS: usize = 240;
pub const MEDIA_IMAGE_ENTITY_ID: &str = "media_image_overlay";
pub const MEDIA_VIDEO_ENTITY_ID: &str = "media_video_overlay";
pub const MEDIA_YOUTUBE_ENTITY_ID: &str = "media_youtube_overlay";
pub const MEDIA_RERUN_ENTITY_ID: &str = "media_rerun_overlay";
const MEDIA_IMAGE_ROUTE_ENTITY_ID: &str = "media_image_expected_route";
const MEDIA_VIDEO_ROUTE_ENTITY_ID: &str = "media_video_expected_route";
const MEDIA_YOUTUBE_ROUTE_ENTITY_ID: &str = "media_youtube_expected_route";
const MEDIA_RERUN_ROUTE_ENTITY_ID: &str = "media_rerun_expected_route";
const IMAGE_URI: &str = "pin.svg";
const VIDEO_URI: &str = "https://cesium.com/public/SandcastleSampleData/big-buck-bunny_trailer.mp4";
const YOUTUBE_VIDEO_ID: &str = "M7lc1UVf-VE";
const RERUN_URI: &str = "https://app.rerun.io/version/0.31.4/examples/dna.rrd";

fn total_steps() -> usize {
    DEMO_INTERVAL_SECONDS
}

struct SamplePosition {
    image_lon: f64,
    image_lat: f64,
    video_lon: f64,
    video_lat: f64,
    youtube_lon: f64,
    youtube_lat: f64,
    rerun_lon: f64,
    rerun_lat: f64,
}

fn sample_positions(step: usize) -> SamplePosition {
    let progress = step as f64 / total_steps() as f64;
    let angle = progress * std::f64::consts::TAU;

    let image_lon = -122.4700 + angle.cos() * 0.018 + (angle * 1.4).sin() * 0.0025;
    let image_lat = 37.8240 + angle.sin() * 0.009 + (angle * 2.1).cos() * 0.0015;
    let video_lon = -122.4786 + angle.sin() * 0.028 + (angle * 2.0).sin() * 0.004;
    let video_lat = 37.8060 + angle.cos() * 0.010 + (angle * 3.0).cos() * 0.002;

    let youtube_lon = -122.4320 + angle.cos() * 0.016 + (angle * 2.4).sin() * 0.003;
    let youtube_lat = 37.7905 + angle.sin() * 0.012 + (angle * 1.7).cos() * 0.0025;
    let rerun_lon = -122.4560 + angle.cos() * 0.012 + (angle * 2.8).sin() * 0.003;
    let rerun_lat = 37.8110 + angle.sin() * 0.008 + (angle * 1.2).cos() * 0.002;

    SamplePosition {
        image_lon,
        image_lat,
        video_lon,
        video_lat,
        youtube_lon,
        youtube_lat,
        rerun_lon,
        rerun_lat,
    }
}

fn image_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let sample = sample_positions(step);
        samples.extend([simulation_seconds, sample.image_lon, sample.image_lat, 26.0]);
    }

    samples
}

fn video_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let sample = sample_positions(step);
        samples.extend([simulation_seconds, sample.video_lon, sample.video_lat, 20.0]);
    }

    samples
}

fn youtube_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let sample = sample_positions(step);
        samples.extend([
            simulation_seconds,
            sample.youtube_lon,
            sample.youtube_lat,
            36.0,
        ]);
    }

    samples
}

fn rerun_samples() -> Vec<f64> {
    let mut samples = Vec::with_capacity((total_steps() + 1) * 4);

    for step in 0..=total_steps() {
        let simulation_seconds = step as f64 * SIM_SECONDS_PER_STEP;
        let sample = sample_positions(step);
        samples.extend([simulation_seconds, sample.rerun_lon, sample.rerun_lat, 44.0]);
    }

    samples
}

fn expected_image_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let sample = sample_positions(step);
        positions.push(sample.image_lon);
        positions.push(sample.image_lat);
        positions.push(26.0);
    }

    positions
}

fn expected_video_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let sample = sample_positions(step);
        positions.push(sample.video_lon);
        positions.push(sample.video_lat);
        positions.push(20.0);
    }

    positions
}

fn expected_youtube_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let sample = sample_positions(step);
        positions.push(sample.youtube_lon);
        positions.push(sample.youtube_lat);
        positions.push(36.0);
    }

    positions
}

fn expected_rerun_route_positions() -> Vec<f64> {
    let mut positions = Vec::with_capacity((total_steps() + 1) * 3);

    for step in 0..=total_steps() {
        let sample = sample_positions(step);
        positions.push(sample.rerun_lon);
        positions.push(sample.rerun_lat);
        positions.push(44.0);
    }

    positions
}

pub fn media_demo_czml() -> String {
    let epoch = CZML_START_EPOCH;
    let image_positions = image_samples();
    let video_positions = video_samples();
    let youtube_positions = youtube_samples();
    let rerun_positions = rerun_samples();
    let image_route = expected_image_route_positions();
    let video_route = expected_video_route_positions();
    let youtube_route = expected_youtube_route_positions();
    let rerun_route = expected_rerun_route_positions();

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
            "id": MEDIA_IMAGE_ENTITY_ID,
            "name": "Image Overlay",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": image_positions,
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "point": {
                "pixelSize": 12,
                "color": {
                    "rgba": [80, 216, 144, 255]
                },
                "outlineColor": {
                    "rgba": [15, 23, 42, 255]
                },
                "outlineWidth": 2
            },
            "properties": {
                "media_kind": "image",
                "media_uri": IMAGE_URI,
                "media_resizable": true,
                "media_width": 160,
                "media_height": 200
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": DEMO_INTERVAL_SECONDS as f64,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [80, 216, 144, 180]
                        }
                    }
                }
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
                "media_uri": VIDEO_URI,
                "media_resizable": true,
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
                "media_youtube_id": YOUTUBE_VIDEO_ID,
                "media_resizable": true,
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
            "id": MEDIA_RERUN_ENTITY_ID,
            "name": "Rerun Overlay",
            "position": {
                "epoch": epoch,
                "cartographicDegrees": rerun_positions,
                "interpolationAlgorithm": "LAGRANGE",
                "interpolationDegree": 1,
                "forwardExtrapolationType": "HOLD",
                "forwardExtrapolationDuration": 0.0
            },
            "point": {
                "pixelSize": 14,
                "color": {
                    "rgba": [104, 155, 255, 255]
                },
                "outlineColor": {
                    "rgba": [15, 23, 42, 255]
                },
                "outlineWidth": 2
            },
            "properties": {
                "media_kind": "rerun",
                "media_uri": RERUN_URI,
                "media_resizable": true,
                "media_width": 360,
                "media_height": 224
            },
            "path": {
                "show": true,
                "width": 4,
                "leadTime": 0,
                "trailTime": DEMO_INTERVAL_SECONDS as f64,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [104, 155, 255, 190]
                        }
                    }
                }
            }
        },
        {
            "id": MEDIA_IMAGE_ROUTE_ENTITY_ID,
            "name": "Image Expected Route",
            "polyline": {
                "positions": {
                    "cartographicDegrees": image_route
                },
                "width": 2,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [80, 216, 144, 150]
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
        },
        {
            "id": MEDIA_RERUN_ROUTE_ENTITY_ID,
            "name": "Rerun Expected Route",
            "polyline": {
                "positions": {
                    "cartographicDegrees": rerun_route
                },
                "width": 2,
                "material": {
                    "solidColor": {
                        "color": {
                            "rgba": [104, 155, 255, 170]
                        }
                    }
                }
            }
        }
    ])
    .to_string()
}
