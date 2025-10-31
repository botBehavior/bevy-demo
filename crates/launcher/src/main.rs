use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImagePlugin;
use bevy::window::WindowPlugin;
use threadweaver_gameplay::GameplayPlugin;
use threadweaver_ui::ThreadweaverUiPlugin;

#[cfg(target_arch = "wasm32")]
use bevy::winit::WinitSettings;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    let backends = if cfg!(target_arch = "wasm32") {
        Backends::BROWSER_WEBGPU
    } else {
        Backends::PRIMARY | Backends::SECONDARY
    };

    app.insert_resource(ClearColor(Color::srgba(0.01, 0.01, 0.015, 1.0)))
        .insert_resource(WgpuSettings {
            backends: Some(backends),
            ..Default::default()
        })
        .add_plugins(default_plugins())
        .add_plugins((GameplayPlugin, ThreadweaverUiPlugin));

    #[cfg(target_arch = "wasm32")]
    {
        app.insert_resource(WinitSettings::game());
    }

    app.run();
}

fn default_plugins() -> PluginGroupBuilder {
    let window = Window {
        title: "Threadweaver".into(),
        canvas: Some("#bevy-canvas".into()),
        fit_canvas_to_parent: true,
        present_mode: bevy::window::PresentMode::AutoVsync,
        prevent_default_event_handling: true,
        ..Default::default()
    };

    let mut plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(window),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            watch_for_changes: false,
            ..Default::default()
        })
        .build();

    #[cfg(target_arch = "wasm32")]
    {
        plugins = plugins.disable::<bevy::log::LogPlugin>();
    }

    plugins
}
