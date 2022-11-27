use common::traits::DrawModel;
use nannou::{color::named, prelude::*};

mod models;
use models::Model;

mod art_bits;
mod common;
mod events;

fn main() {
    nannou::app(model).event(event).run()
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();

    let rect = app.window_rect();

    Model::new(window, &rect)
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::S) => events::screenshot(app, model.seed.into()),
                    KeyReleased(Key::R) => events::reset(model),
                    KeyReleased(Key::Up) => events::change_num_squares(50, model),
                    KeyReleased(Key::Down) => events::change_num_squares(-50, model),
                    _ => (),
                }
            }
        }

        Event::Update(update) => events::update(app, model, update),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(named::CORNSILK);

    if app.elapsed_frames() % 60 == 0 {
        println!("num squares: {}", model.squares.len());
    }

    model.squares.iter().for_each(|s| s.draw(&draw));

    draw.to_frame(app, &frame).unwrap();
}
