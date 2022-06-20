use bevy::prelude::*;

use super::{
    player_core_tile::{PLAYER_TILE_SIZE, PLAYER_TILE_Z},
    Player,
};

#[derive(Component)]
pub struct PlayerExtraTile {}

#[derive(Bundle)]
struct PlayerExtraTileBundle {
    tile: PlayerExtraTile,
    player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl PlayerExtraTile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(&self, location: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        let tile = PlayerExtraTile::new();
        let player = Player::new();

        let texture = asset_server.load("textures/block_connection.png");

        let sprite_bundle = SpriteBundle {
            texture: texture,
            transform: Transform::from_xyz(location.x, location.y, PLAYER_TILE_Z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE)),
                ..Default::default()
            },
            ..default()
        };

        let player_bundle = PlayerExtraTileBundle {
            tile,
            player,
            sprite_bundle,
        };

        commands.spawn_bundle(player_bundle);
    }
}
