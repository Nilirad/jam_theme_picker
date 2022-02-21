use bevy::prelude::*;
use theme_picker_lib::*;

fn main() {
    App::new().add_plugin(ThemePickerPlugin).run();
}
