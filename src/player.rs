use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::{ConfigureLoadingState, LoadingStateConfig};
use bevy_asset_loader::loading_state::LoadingStateAppExt;

use crate::{GameState, Player};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_character)
            .add_systems(Update, fire_projectiles);
    }
}

fn fire_projectiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1., 1.))),
            material: materials.add(Color::ORANGE_RED),
            transform: Transform::from_xyz(0., 0., 5.),
            ..default()
        });
    }
}

fn spawn_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50. })),
            material: materials.add(Color::ORANGE_RED),
            transform: Transform::from_xyz(0., 0., 5.),
            ..default()
        },
        Player,
    ));

    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(10., 10., 0.),
            ..default()
        }),
    );
}
/*
#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 12, rows = 12))]
    playe
    #[asset(path = "images/player.png")]
    sprite: Handle<Image>,
}
*/
