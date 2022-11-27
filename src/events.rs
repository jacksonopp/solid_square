use std::fmt::format;

use crate::models::Model;
use nannou::prelude::*;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Do stuff
}

pub fn screenshot(app: &App, seed: usize) {
    let ext = format!("-seed-{}", seed);

    app.main_window()
        .capture_frame("images/".to_string() + &app.exe_name().unwrap_or("file-".to_owned()) + ext.as_str() + ".png");
}
