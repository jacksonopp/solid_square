use crate::models::Model;
use nannou::prelude::*;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Do stuff
}

pub fn reset(model: &mut Model) {
    println!("Resetting...");
    model.reset();
    println!("Model reset!");
}

pub fn screenshot(app: &App, seed: usize) {
    let app_name = &app.exe_name().unwrap_or("file".to_owned());
    let ext = format!("images/{}-{}.png", app_name, seed);

    app.main_window()
        .capture_frame(ext);
}

pub fn change_num_squares(by: i32, model: &mut Model) {
    model.change_num_squares(by).unwrap_or_else(|e| {
        println!("{}", e);
    })
}