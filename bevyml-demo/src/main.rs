use bevy::{
    ecs::relationship::RelatedSpawnerCommands,
    log::{DEFAULT_FILTER, Level, LogPlugin},
    prelude::*,
};
use bevyml::{BevyNodeTree, BevymlAsset, BevymlAssetPlugin};

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
                spawn_tree(&mut commands, root);
            }

            *spawned = true;
        }
        None => bevy::log::error!("Failed to load UI root."),
    }
}

fn spawn_tree(commands: &mut Commands, tree: &BevyNodeTree) {
    let mut entity = commands.spawn(tree.node.clone());
    if let Some(text) = tree.text.clone() {
        entity.insert(text);
    }
    entity.with_children(|parent| {
        for child in &tree.children {
            spawn_tree_child(parent, child);
        }
    });
}

fn spawn_tree_child(parent: &mut RelatedSpawnerCommands<'_, ChildOf>, tree: &BevyNodeTree) {
    let mut entity = parent.spawn(tree.node.clone());
    if let Some(text) = tree.text.clone() {
        entity.insert(text);
    }
    entity.with_children(|parent| {
        for child in &tree.children {
            spawn_tree_child(parent, child);
        }
    });
}
