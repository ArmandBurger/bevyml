use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevyml::{BevyNodeTree, BevymlAsset, BevymlAssetPlugin};

fn main() {
    let asset_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .to_string_lossy()
        .into_owned();

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_root,
            ..default()
        }))
        .add_plugins(BevymlAssetPlugin)
        .init_resource::<UiState>()
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_on_load)
        .run();
}

#[derive(Resource, Default)]
struct UiState {
    handle: Handle<BevymlAsset>,
    spawned: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut state: ResMut<UiState>) {
    commands.spawn(Camera2d);
    state.handle = asset_server.load("ui/demo.bevyml");
}

fn spawn_on_load(
    mut commands: Commands,
    mut state: ResMut<UiState>,
    assets: Res<Assets<BevymlAsset>>,
) {
    if state.spawned {
        return;
    }

    let Some(asset) = assets.get(&state.handle) else {
        return;
    };

    for root in &asset.roots {
        spawn_tree(&mut commands, root);
    }

    state.spawned = true;
}

fn spawn_tree(commands: &mut Commands, tree: &BevyNodeTree) -> Entity {
    let entity = commands.spawn(tree.node.clone()).id();
    for child in &tree.children {
        let child_entity = spawn_tree(commands, child);
        commands.entity(entity).add_child(child_entity);
    }
    entity
}
