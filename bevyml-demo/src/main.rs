use bevy::{
    color::palettes::tailwind,
    log::{DEFAULT_FILTER, Level, LogPlugin},
    prelude::*,
};
use bevyml::{BevymlAsset, BevymlAssetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::INFO,
            filter: format!(
                "{},bevyml_demo=trace,bevyml_parser=trace,wgpu_hal=warn",
                DEFAULT_FILTER
            ),
            ..default()
        }))
        .add_plugins(BevymlAssetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_ui)
        .run();
}

#[derive(Resource, Default, Deref)]
pub struct BevymlUI(Handle<BevymlAsset>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera));

    let document = asset_server.load::<BevymlAsset>("ui/demo.bevyml");
    commands.insert_resource(BevymlUI(document));
}

fn spawn_ui(
    mut spawned: Local<bool>,
    mut commands: Commands,
    res: ResMut<Assets<BevymlAsset>>,
    ui: ResMut<BevymlUI>,
) {
    if *spawned {
        return;
    }

    match res.get(&ui.0) {
        Some(ml) => {
            let roots = &ml.roots;

            for root in roots {
                dbg!(&root.node.node);
                commands.spawn((
                    root.node.name.clone(),
                    root.node.node_kind.clone(),
                    root.node.node.clone(),
                    BackgroundColor(tailwind::BLUE_400.into()),
                ));
            }

            *spawned = true;
        }
        None => bevy::log::error!("Failed to load UI root."),
    }
}
