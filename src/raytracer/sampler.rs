use rand::{thread_rng, Closed01, Rng};

pub struct Random;
pub struct Jittered;
pub struct Uniform;

pub trait PixelSampler {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)>;
}

impl PixelSampler for Uniform {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)> {
        let step = 1. / n as f64;
        let mut v = vec!();
        let (px, py) = pixel;
        let mut i = 0;
        while i != n {
            let mut j = 0;
            while j != n {
                v.push((px as f64 + (0.5 + i as f64) * step,
                        py as f64 + (0.5 + j as f64) * step));
                j += 1;
            }
            i += 1;
        }

        v
    }
}

impl PixelSampler for Random {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)> {
        let (px, py) = pixel;
        let mut rng = thread_rng();
        let n2 = n * n;
        (0..n2).into_iter().map(|_| {
            let (Closed01(x), Closed01(y)) = (rng.gen::<Closed01<f64>>(),
                                              rng.gen::<Closed01<f64>>());
            (px as f64 + x, py as f64 + y)
        }).collect()
    }
}

impl PixelSampler for Jittered {
    fn samples(&mut self, pixel: (u32, u32), n: u32) -> Vec<(f64, f64)> {
        let step = 1. / n as f64;
        let mut v = vec!();
        let (px, py) = pixel;
        let mut rng = thread_rng();
        let mut i = 0;
        while i != n {
            let mut j = 0;
            while j != n {
                let (Closed01(x), Closed01(y)) = (rng.gen::<Closed01<f64>>(),
                                                  rng.gen::<Closed01<f64>>());
                v.push((px as f64 + (x + i as f64) * step,
                        py as f64 + (y + j as f64) * step));
                j += 1;
            }
            i += 1;
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sampler_in_range<S: PixelSampler>(mut sampler: S) {
        let samples = sampler.samples((10, 10), 3);
        for (x, y) in samples {
            assert!(10. <= x && x <= 11. && 10. <= y && y <= 11.);
        }
    }

    #[test]
    fn test_random_sampler_in_range() {
        test_sampler_in_range(Random);
    }

    #[test]
    fn test_jittered_sampler_in_range() {
        test_sampler_in_range(Jittered);
    }

    #[test]
    fn test_uniform_sampler_in_range() {
        test_sampler_in_range(Uniform);
    }
}
