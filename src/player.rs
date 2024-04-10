use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::{ConfigureLoadingState, LoadingStateConfig};
use bevy_asset_loader::loading_state::LoadingStateAppExt;

use crate::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        /*app.configure_loading_state(
            LoadingStateConfig::new(GameState::LoadingScreen).load_collection::<PlayerAssets>(),
        );*/
    }
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
