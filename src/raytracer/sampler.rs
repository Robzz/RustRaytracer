use rand::{ThreadRng, thread_rng, Closed01, Rng};

pub struct Random {
    rng: ThreadRng
}

impl Random {
    pub fn new() -> Random {
        Random { rng: thread_rng() }
    }
}

pub trait PixelSampler {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)>;
}

impl PixelSampler for Random {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)> {
        let mut i = 0;
        let mut v = vec!();
        let (px, py) = pixel;
        while i != n {
            let (Closed01(x), Closed01(y)) = (self.rng.gen::<Closed01<f64>>(),
                                              self.rng.gen::<Closed01<f64>>());
            v.push((px as f64 + x, py as f64 + y));
            i += 1;
        }

        v
    }
}
