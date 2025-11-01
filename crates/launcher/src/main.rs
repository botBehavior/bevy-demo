use bevy::app::PluginGroupBuilder;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    texture::ImagePlugin,
    RenderPlugin,
};

#[cfg(target_arch = "wasm32")]
use bevy::render::settings::WgpuSettingsPriority;
use bevy::window::WindowPlugin;
use threadweaver_gameplay::GameplayPlugin;
use threadweaver_ui::ThreadweaverUiPlugin;

#[cfg(target_arch = "wasm32")]
use bevy::winit::WinitSettings;

#[cfg(target_arch = "wasm32")]
use uuid as _;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();
    let backends = if cfg!(target_arch = "wasm32") {
        Backends::BROWSER_WEBGPU | Backends::GL
    } else {
        Backends::PRIMARY | Backends::SECONDARY
    };

    app.insert_resource(ClearColor(Color::srgba(0.01, 0.01, 0.015, 1.0)))
        .add_plugins(default_plugins(backends))
        .add_plugins((GameplayPlugin, ThreadweaverUiPlugin));

    #[cfg(target_arch = "wasm32")]
    {
        app.insert_resource(WinitSettings::game());
    }

    app.run();
}

fn default_plugins(backends: Backends) -> PluginGroupBuilder {
    let window = Window {
        title: "Threadweaver".into(),
        canvas: Some("#bevy-canvas".into()),
        fit_canvas_to_parent: true,
        present_mode: bevy::window::PresentMode::AutoVsync,
        prevent_default_event_handling: true,
        ..Default::default()
    };

    let plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(window),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            watch_for_changes_override: Some(false),
            ..Default::default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(default_wgpu_settings(backends)),
            ..Default::default()
        })
        .build();

    apply_wasm_overrides(plugins)
}

#[cfg(target_arch = "wasm32")]
fn default_wgpu_settings(backends: Backends) -> WgpuSettings {
    let mut settings = WgpuSettings {
        backends: Some(backends),
        ..Default::default()
    };
    settings.priority = WgpuSettingsPriority::WebGL2;
    settings
}

#[cfg(not(target_arch = "wasm32"))]
fn default_wgpu_settings(backends: Backends) -> WgpuSettings {
    WgpuSettings {
        backends: Some(backends),
        ..Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_wasm_overrides(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    plugins.disable::<bevy::log::LogPlugin>()
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_wasm_overrides(plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    plugins
}
