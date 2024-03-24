use rand::Rng;
use either::Either;
use std::iter::once;

use crate::vector::{Vec2, transpose};

#[derive(Debug)]
pub struct Ising {
    pub size: usize,
    pub lattice: Vec<i8>,
    pub temperature: f64,
    pub coupling_constant: f64,
    pub magnetic_moment: f64,
    pub magnetic_field_strength: f64,
}

impl Ising {
    pub fn new(size: usize) -> Self {
        let mut lattice = vec![0 as i8; size.pow(2)];

        for x in lattice.iter_mut() {
            *x = 2 * (rand::random::<f64>().round() as i8) - 1;
        }

        Self {
            size,
            lattice,
            temperature: 2.696,
            coupling_constant: 1.0,
            magnetic_moment: 0.0,
            magnetic_field_strength: 0.0,
        }
    }

    pub fn monte_carlo_step(&mut self, rng: &mut rand::rngs::ThreadRng) {
        let pos = Vec2 {
            x: rng.gen_range(0..self.size) as i32,
            y: rng.gen_range(0..self.size) as i32,
        };

        let energy_diff = -2.0 * self.hamiltonian(&pos);
        if energy_diff <= 0.0 {
            self.flip_spin_at_pos(&pos)
        } else {
            let p = f64::exp(-energy_diff / self.temperature);
            if p <= 1.0 && rand::random::<f64>() < p {
                self.flip_spin_at_pos(&pos);
            }
        }
    }

    fn periodic_bc(&self, index: i32) -> i32 {
        index.rem_euclid(self.size as i32)
    }

    fn get_index(&self, pos: &Vec2) -> usize {
        (pos.y * (self.size as i32) + pos.x) as usize
    }

    fn get_neighbors(&self, pos: &Vec2) -> [i8; 4] {
        [
            self.lattice[self.get_index(&Vec2 {
                x: self.periodic_bc(pos.x + 1),
                y: pos.y,
            })],
            self.lattice[self.get_index(&Vec2 {
                x: self.periodic_bc(pos.x - 1),
                y: pos.y,
            })],
            self.lattice[self.get_index(&Vec2 {
                x: pos.x,
                y: self.periodic_bc(pos.y + 1),
            })],
            self.lattice[self.get_index(&Vec2 {
                x: pos.x,
                y: self.periodic_bc(pos.y - 1),
            })],
        ]
    }

    fn hamiltonian(&self, pos: &Vec2) -> f64 {
        let spin = self.lattice[self.get_index(pos)] as f64;

        let neighbors_spin_sum: f64 = self.get_neighbors(pos).iter().sum::<i8>() as f64;
        let coupling_term: f64 = -self.coupling_constant * spin * neighbors_spin_sum;
        let magnetic_term: f64 = -self.magnetic_moment * self.magnetic_field_strength * spin;

        return coupling_term + magnetic_term;
    }

    fn flip_spin_at_pos(&mut self, pos: &Vec2) {
        let idx = self.get_index(pos);
        self.lattice[idx] *= -1;
    }

    pub fn lattice_as_braille(&self) -> String {
        let width = self.size / 2;
        let height = self.size / 4;
        let num_pixels = width * height;

        let chunk: Vec<&[i8]> = self.lattice.chunks(2).collect();
        let chunk: Vec<Vec<&[i8]>> = chunk.chunks(width).map(|x| x.to_vec()).collect();
        let chunk_t = transpose(chunk);

        let mut i = 0;
        let mut pixels_t = vec![[0; 8]; num_pixels];

        for col in chunk_t {
            let pixel_chunk = col.chunks(4);
            for pixel in pixel_chunk {
                let mut flat_pixel: [u8; 8] = [0; 8];
                let mut j: usize = 0;
                for b in pixel {
                    flat_pixel[j] = ((b[0] + 1) / 2) as u8;
                    flat_pixel[j + 1] = ((b[1] + 1) / 2) as u8;
                    j += 2;
                }
                pixels_t[i] = flat_pixel;
                i += 1;
            }
        }

        // TRANSPOSE PIXELS
        let mut pixels = vec![[0; 8]; num_pixels];
        for i in 0..width {
            for j in 0..height {
                pixels[i + j * width] = pixels_t[i * height + j];
            }
        }

        // Convert to braille encoding
        let encoding = [0, 3, 1, 4, 2, 5, 6, 7];
        let pixels: Vec<char> = pixels
            .iter()
            .map(|pixel| {
                let mut pixel_braille: u8 = 0;
                for (pixel_b, encoding_b) in pixel.iter().zip(encoding.iter()) {
                    pixel_braille += pixel_b << encoding_b;
                }
                char::from_u32(10240 + pixel_braille as u32).unwrap()
            })
            .collect();

        // Unflatten and convert to string
        pixels.into_iter()
            .enumerate()
            .flat_map(|(i, c)| {
                if i % width == 0 {
                    Either::Left(['\n', c].into_iter())
                } else {
                    Either::Right(once(c))
                }
            })
            .skip(1) // first Either::Left creates a leading \n so skip it before collecting
            .collect::<String>()
    }
}
