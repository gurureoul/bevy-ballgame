pub mod resources;
mod systems;

use crate::game::score::resources::*;
use crate::game::score::systems::*;
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(high_scores_updated);
    }
}
