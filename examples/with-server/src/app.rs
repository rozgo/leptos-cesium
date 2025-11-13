use leptos::prelude::*;
use leptos_cesium::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="Cesium/Widgets/widgets.css" />
                <script src="Cesium/Cesium.js"></script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
                <style>
                    "html, body { margin: 0; width: 100%; height: 100%; background: #0b0d18; color: #e0e4ff; font-family: sans-serif; overflow: hidden; }"
                </style>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Load Cesium Ion token from environment at build time
    let ion_token = option_env!("CESIUM_ION_TOKEN").map(|s| s.to_string());

    view! {
        <Title text="Leptos Cesium with Server"/>
        <Router>
            <main>
                <Routes fallback=|| "This page couldn't be found">
                    <Route path=path!("") view=move || view! {
                        <ViewerContainer
                            ion_token=ion_token.clone()
                            animation=false
                            timeline=false
                            style="width: 100%; height: 100%;".to_string()
                        >
                            <Entity
                                name=Some("Statue of Liberty".to_string())
                                position=Some(Cartesian3::from_degrees(-74.0445, 40.6892, 150.0))
                            >
                                <PointGraphics
                                    pixel_size=12.0
                                    color=Some(Color::red())
                                />
                            </Entity>
                        </ViewerContainer>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}
