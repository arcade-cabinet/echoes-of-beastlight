pub mod seed;
pub mod lexicon;
pub mod tiles;
pub mod generation;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                seed::SeedPlugin,
                lexicon::LexiconPlugin,
                tiles::TilePlugin,
                generation::GenerationPlugin,
            ));
    }
}