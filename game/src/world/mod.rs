pub mod generation;
pub mod lexicon;
pub mod seed;
pub mod tiles;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            seed::SeedPlugin,
            lexicon::LexiconPlugin,
            tiles::TilePlugin,
            generation::GenerationPlugin,
        ));
    }
}
