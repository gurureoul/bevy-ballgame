pub mod enemy;
mod events;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;
use bevy::prelude::*;
use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
