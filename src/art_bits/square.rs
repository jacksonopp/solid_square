use crate::common::traits::DrawModel;

use nannou::{prelude::*, rand::{seq::SliceRandom, rngs::StdRng}};

pub struct Square {
    pub w: f32,
    pub pos: Point2,
    pub fill: Hsl,
    pub border_color: Hsl,
    pub border_width: f32 
}

impl Square {
    pub fn new(w: f32, pos: Point2, rng: &mut StdRng) -> Self {
        let colors = [
            hsl(hue(46.0), 0.27, 0.65),
            hsl(hue(288.0), 0.06, 0.33),
            hsl(hue(354.0), 0.3, 0.44),
            hsl(hue(40.0), 0.26, 0.93),
            hsl(hue(210.0), 0.03, 0.15),
        ];

        let fill = *colors.choose(rng).unwrap();
        let border_color = *colors.choose(rng).unwrap();
        let border_width = 1.0;

        Self {
            w,
            pos,
            fill,
            border_color,
            border_width
        }
    }
}

impl DrawModel for Square {
    fn draw(&self, draw: &nannou::Draw) {
        draw.rect()
            .stroke(self.border_color)
            .stroke_weight(self.border_width)
            .w_h(self.w, self.w)
            .xy(self.pos)
            .color(self.fill);
    }
}

fn hue(n: f32) -> f32 {
    n / 360.0
}