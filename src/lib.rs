#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

use bevy::prelude::*;
use randomorg::Random;

/// A plugin
pub struct ThemePickerPlugin;

impl Plugin for ThemePickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Themes>()
            .add_startup_system(setup_themes)
            .add_system(pick_theme);
    }
}

#[derive(Debug, Default)]
struct Themes(Vec<String>);

fn setup_themes(mut themes: ResMut<Themes>) {
    if std::env::args().len() < 2 {
        panic!("You must provide at least two themes!");
    }

    themes.0 = std::env::args().collect();
}

fn pick_theme(themes: Res<Themes>) {
    // random.org terms of use didn't mention anything about sharing API keys, so...
    let api_key = include_str!("random-api-key.txt");
    let rng = Random::new(api_key);
    let max_index = (themes.0.len() as i32) - 1;
    let index = rng
        .generate_integers(1, max_index, 1, true)
        .expect("Failed to retrieve a random value from random.org.")
        .result
        .random
        .data
        .first()
        .copied()
        .expect("Response from random.org is empty!") as usize;

    println!("The chosen theme is: {}", themes.0[index]);
}
