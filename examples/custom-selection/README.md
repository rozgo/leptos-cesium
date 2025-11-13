# Custom Selection Panel Example

This example demonstrates how to create custom selection panels in leptos-cesium by disabling Cesium's default InfoBox widget and handling entity selection reactively with Leptos signals.

## Features Demonstrated

- **Disabling Default InfoBox**: Set `info_box=false` on `ViewerContainer`
- **Reactive Selection Tracking**: Access selected entity via `viewer_context.selected_entity_signal()`
- **Custom Panel UI**: Build your own selection panel with custom styling
- **Entity Property Access**: Read entity name, description, ID, and position
- **Programmatic Selection Control**: Clear selection programmatically

## Key Concepts

### 1. Disable Default InfoBox

```rust
<ViewerContainer
    info_box=false  // Disable Cesium's default InfoBox
    selection_indicator=true  // Keep the green selection indicator
>
    // Your content
</ViewerContainer>
```

### 2. Access Selected Entity Reactively

```rust
#[component]
fn CustomSelectionPanel() -> impl IntoView {
    let viewer_context = use_cesium_context().expect("Must be inside ViewerContainer");

    // Get reactive signal for selected entity
    let selected_signal = viewer_context.selected_entity_signal();

    view! {
        <Show when=move || selected_signal.get().is_some()>
            {move || {
                viewer_context.selected_entity().map(|entity| {
                    // Access entity properties
                    let name = entity.name();
                    let description = entity.description();
                    let id = entity.id();

                    view! {
                        <div>
                            <h3>{extract_string_value(&name)}</h3>
                            <p>{extract_string_value(&description)}</p>
                            <small>"ID: " {id}</small>
                        </div>
                    }
                })
            }}
        </Show>
    }
}
```

### 3. Clear Selection Programmatically

```rust
viewer_context.clear_selected_entity();
```

## Entity Properties API

The `Entity` type provides strongly-typed access to common properties:

```rust
let entity: Entity = viewer_context.selected_entity().unwrap();

// Strongly-typed accessors
entity.id()          // String - unique entity identifier
entity.name()        // JsValue - entity name (may be a Property)
entity.description() // JsValue - entity description (may be a Property)
entity.position()    // JsValue - entity position (Cartesian3 or PositionProperty)
entity.properties()  // JsValue - custom properties bag
```

## Running the Example

### Prerequisites

1. Ensure Cesium assets are synced:
   ```bash
   # From repository root
   ./scripts/sync_cesium_assets.sh
   ```

2. Set up your Cesium Ion token:
   ```bash
   # Copy .env.example to .env.local and add your token
   cp .env.example .env.local
   # Edit .env.local and add: CESIUM_ION_TOKEN=your_token_here
   ```

### Run with Trunk

```bash
cd examples/custom-selection
trunk serve --open
```

The example will open in your browser at http://localhost:8080

## UI Features

- **Custom Panel**: Displays entity details in a styled panel (top-right)
- **Instructions**: Helpful guide explaining the example (top-left, dismissible)
- **Close Buttons**: Both panels can be dismissed by clicking the × button
- **Styling**: Modern, polished UI with proper typography and spacing

## Entities in the Scene

The example creates several entities to demonstrate selection:

1. **Red Building** - A tall box with red color
2. **Blue Sphere** - A floating spherical monument
3. **Green Cylinder Tower** - A cylindrical structure
4. **Yellow Plaza** - A rectangular area with checkerboard pattern
5. **Purple Path** - A glowing polyline path

Click on any of these to see the custom selection panel in action!

## Implementation Details

### Strongly-Typed Entity Access

The implementation uses strongly-typed `Entity` objects instead of generic `JsValue`:

```rust
// Context provides strongly-typed access
pub fn selected_entity(&self) -> Option<Entity> { ... }

// Event listener updates context internally
viewer.selected_entity_changed().add_event_listener(...);
```

### Handling Cesium Properties

Cesium entities often use `Property` objects that need to be evaluated:

```rust
fn extract_string_value(value: &JsValue) -> String {
    // Check if it's a Property with getValue() method
    if let Ok(get_value_fn) = /* get getValue function */ {
        return get_value_fn.call0(value).as_string();
    }
    // Otherwise convert directly
    value.as_string().unwrap_or("N/A".to_string())
}
```

## Next Steps

- Add support for editing entity properties through the panel
- Display entity graphics information (box dimensions, colors, etc.)
- Add buttons to fly to selected entity or track it
- Show entity's property values over time for animated entities

## Learn More

- [Cesium Entity Documentation](https://cesium.com/learn/cesiumjs/ref-doc/Entity.html)
- [leptos-cesium Documentation](../../README.md)
- [Leptos Signals Guide](https://book.leptos.dev/reactivity/index.html)
