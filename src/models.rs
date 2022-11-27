use nannou::{
    prelude::*,
    rand::{rngs::StdRng, SeedableRng},
};
use rand_distr::{Normal, Distribution};

use crate::art_bits::square::Square;

pub struct Model {
    pub window: WindowId,
    pub squares: Vec<Square>,
    pub seed: u16,
}

const NUM_SQUARES: u32 = 500;

impl Model {
    pub fn new(w: WindowId, rect: &Rect) -> Model {
        let mut squares = vec![];

        let (width, height) = rect.w_h();
        println!("w: {width}; h: {height}");

        let seed = random::<u16>();
        println!("seed: {seed}");
        // let mut rng = StdRng::seed_from_u64(seed as u64);
        let mut rng = StdRng::seed_from_u64(1);
        let nh = Normal::new(0.0, 350.0).unwrap();
        let nv = Normal::new(0.0, 350.0).unwrap();
        let nw = Normal::new(25.0, 10.0).unwrap();

        for _ in 0..NUM_SQUARES {
            let pos_x = nh.sample(&mut rng);
            let pos_y = nv.sample(&mut rng);
            // let w = random_range(10.0, 50.0);
            let w = nw.sample(&mut rng);
            let sq = Square::new(w, vec2(pos_x, pos_y), &mut rng);
            squares.push(sq);
        }

        Model { window: w, squares, seed}
    }
}
