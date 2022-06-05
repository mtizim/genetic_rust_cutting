#![allow(unused)]
use std::iter::zip;

use basic_structs::{Circle, Pos, Rect, ViableRect, ViableRectData};
mod basic_structs;
use rand_distr::{Distribution, Normal, Uniform};

static RADIUS: f32 = 2.0;
static HALFRADIUS: f32 = RADIUS / 2.0;
static CIRCLE: Circle = Circle { radius: 2.0 };
fn main() {
    println!("Hello, world!");
}

struct Chromosome {
    pub rects: Vec<Rect>,
}

impl Chromosome {
    pub fn new() -> Chromosome {
        Chromosome { rects: Vec::new() }
    }
    pub fn from_halves(mut left: Vec<Rect>, right: Vec<Rect>) -> Chromosome {
        left.extend(right);
        Chromosome { rects: left }
    }

    // viable_rects - sorted ascending according to value
    pub fn mutate(&mut self, rect_data: &ViableRectData) -> Chromosome {
        // mutate existing
        {
            let rng = &mut rand::thread_rng();
            let probas = Uniform::new(0.0, 1.0).sample_iter(rng);
            for (i, prob) in zip(0..self.rects.len(), probas) {
                if prob < self.rects[i].mut_prob {
                    for viable_rect in rect_data.rects {
                        if viable_rect.value <= self.rects[i].value {
                            continue;
                        } else {
                            let new_rect = Rect::new(
                                &self.rects[i].center,
                                viable_rect.height,
                                viable_rect.width,
                                viable_rect.value,
                                rect_data.min_value,
                                rect_data.max_value,
                            );
                            if (self.rects[i].covers(viable_rect) || {
                                // the hard case
                                let rect = &self.rects[i];
                                !self
                                    .rects
                                    .iter()
                                    // get viable overlaps
                                    .filter(|r| {
                                        (rect.center.x - r.center.x).abs() <= rect_data.max_width
                                            && (rect.center.y - r.center.y).abs()
                                                <= rect_data.max_height
                                    })
                                    .any(|r| r.overlaps(&new_rect))
                            }) {
                                self.rects[i] = new_rect;
                            }
                        }
                    }
                }
            }
        }
        //  attempt n insertions
        {
            let new_rect_attempts = 15;
            let rng = &mut rand::thread_rng();
            let mut sampler = Uniform::new(-CIRCLE.radius, CIRCLE.radius).sample_iter(rng);
            for _ in 0..new_rect_attempts {
                let center = Pos {
                    x: sampler.next().unwrap(),
                    y: sampler.next().unwrap(),
                };
                if !CIRCLE.contains_point(&center) {
                    continue;
                }
                for i in 0..self.rects.len() {
                    for viable_rect in rect_data.rects.iter().rev() {
                        let new_rect = Rect::new(
                            &center,
                            viable_rect.height,
                            viable_rect.width,
                            viable_rect.value,
                            rect_data.min_value,
                            rect_data.max_value,
                        );
                        let res = {
                            !self
                                .rects
                                .iter()
                                .filter(|r| {
                                    (center.x - r.center.x).abs() <= rect_data.max_width
                                        && (center.y - r.center.y).abs() <= rect_data.max_height
                                })
                                .any(|r| r.overlaps(&new_rect))
                        };
                        if res {
                            self.rects.push(new_rect);
                        }
                    }
                }
            }
        }
        todo!()
    }
    pub fn crossover(self, other: Chromosome) -> (Chromosome, Chromosome) {
        let slice_distr = Normal::new(0.0, HALFRADIUS).unwrap();
        let slice_x = slice_distr
            .sample(&mut rand::thread_rng())
            .clamp(-CIRCLE.radius * 0.9, CIRCLE.radius * 0.9);

        let self_without_line = self
            .rects
            .into_iter()
            .filter(|r| !r.contains_vert_line(slice_x));
        let other_without_line = other
            .rects
            .into_iter()
            .filter(|r| !r.contains_vert_line(slice_x));

        let mut self_left = Vec::new();
        let mut self_right = Vec::new();
        for r in self_without_line {
            if (r.center.x <= slice_x) {
                self_left.push(r)
            } else {
                self_right.push(r)
            }
        }
        let mut other_left = Vec::new();
        let mut other_right = Vec::new();
        for r in other_without_line {
            if (r.center.x <= slice_x) {
                other_left.push(r)
            } else {
                other_right.push(r)
            }
        }

        (
            Chromosome::from_halves(self_left, other_right),
            Chromosome::from_halves(other_left, self_right),
        )
    }
}
