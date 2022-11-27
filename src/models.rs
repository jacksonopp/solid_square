use nannou::{
    prelude::*,
    rand::{rngs::StdRng, SeedableRng},
};
use rand_distr::{Distribution, Normal};

use crate::art_bits::square::Square;

pub struct Model {
    pub window: WindowId,
    pub squares: Vec<Square>,
    pub seed: u16,
    pub num_sq: i32,
}

const NUM_SQUARES: i32 = 500;

impl Model {
    pub fn new(w: WindowId, rect: &Rect) -> Model {
        let (width, height) = rect.w_h();
        println!("w: {width}; h: {height}");

        let seed = random::<u16>();
        println!("seed: {seed}");
        let nh = Normal::new(0.0, 350.0).unwrap();
        let nv = Normal::new(0.0, 350.0).unwrap();
        let nw = Normal::new(25.0, 10.0).unwrap();

        let squares = gen_squares(nh, seed.into(), nv, nw, NUM_SQUARES);

        Model {
            window: w,
            squares,
            seed,
            num_sq: NUM_SQUARES,
        }
    }

    pub fn reset(&mut self) {
        let seed = random::<u16>();
        println!("seed: {seed}");
        let nh = Normal::new(0.0, 350.0).unwrap();
        let nv = Normal::new(0.0, 350.0).unwrap();
        let nw = Normal::new(25.0, 10.0).unwrap();

        self.seed = seed;

        let squares = gen_squares(nh, self.seed.into(), nv, nw, self.num_sq);
        self.squares = squares;
    }

    pub fn change_num_squares(&mut self, by: i32) -> Result<(), String> {
        let num_sq = self.num_sq + by;
        if num_sq <= 0 {
            return Err("Oopsie".to_string());
        };

        self.num_sq = num_sq;

        let nh = Normal::new(0.0, 350.0).unwrap();
        let nv = Normal::new(0.0, 350.0).unwrap();
        let nw = Normal::new(25.0, 10.0).unwrap();

        self.squares = gen_squares(nh, self.seed.into(), nv, nw, self.num_sq);

        

        Ok(())
    }
}

fn gen_squares(
    nh: Normal<f32>,
    seed: u64,
    nv: Normal<f32>,
    nw: Normal<f32>,
    num_sq: i32,
) -> Vec<Square> {
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let mut squares = vec![];
    for _ in 0..num_sq {
        let pos_x = nh.sample(&mut rng);
        let pos_y = nv.sample(&mut rng);
        // let w = random_range(10.0, 50.0);
        let w = nw.sample(&mut rng);
        let sq = Square::new(w, vec2(pos_x, pos_y), &mut rng);
        squares.push(sq);
    }
    squares
}
