use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub bird_down_handle: Handle<Image>,
    pub bird_mid_handle: Handle<Image>,
    pub bird_up_handle: Handle<Image>,

    pub pipe: Handle<Image>,
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bird_down_handle: asset_server.load("bluebird-downflap.png"),
        bird_mid_handle: asset_server.load("bluebird-midflap.png"),
        bird_up_handle: asset_server.load("bluebird-upflap.png"),
        pipe: asset_server.load("pipe-green.png"),
    })
}
