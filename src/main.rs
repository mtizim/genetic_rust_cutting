#![allow(unused)]
use std::{iter::zip, path::Path};

use basic_structs::{Circle, Pos, Rect, TaskData, ViableRect};
use rand::prelude::SliceRandom;
use rayon::prelude::*;
mod basic_structs;
mod tests;
use rand_distr::{Distribution, Normal, Uniform};

fn main() {
    {
        // around 2000 iters
        println!("R800");
        let mut evo = Evolution::new(256, 800_f32, "cutting/r800.csv");
        for i in 0..50000 {
            evo.advance();
            let res = evo.get_best_result();
            if res >= 30000.0 {
                println!("Found r800, score {} in iter {}", res, i);
                break;
            }
            if i % 1000 == 0 {
                println!("score {} in iter {}", res, i);
            }
        }
    }
    {
        // doesn't finish in 5000 iters, max score 9540
        println!("R1200");
        let mut evo = Evolution::new(256, 1200_f32, "cutting/r1200.csv");
        for i in 0..50000 {
            evo.advance();
            let res = evo.get_best_result();
            if res >= 30000.0 {
                println!("Found r1200, score {} in iter {}", res, i);
                break;
            }
            if i % 1000 == 0 {
                println!("score {} in iter {}", res, i);
            }
        }
    }
    {
        // around 2000 iters
        println!("R1000");
        let mut evo = Evolution::new(256, 1000_f32, "cutting/r1000.csv");
        for i in 0..50000 {
            evo.advance();
            let res = evo.get_best_result();
            if res >= 17500.0 {
                println!("Found r1000, score {} in iter {}", res, i);
                break;
            }
            if i % 1000 == 0 {
                println!("score {} in iter {}", res, i);
            }
        }
    }
    {
        println!("R1100");
        let mut evo = Evolution::new(256, 1100_f32, "cutting/r1100.csv");
        for i in 0..50000 {
            evo.advance();
            let res = evo.get_best_result();
            if res >= 25000.0 {
                println!("Found r1000, score {} in iter {}", res, i);
                break;
            }
            if i % 1000 == 0 {
                let res = evo.get_best_result();
                println!("score {} in iter {}", res, i);
            }
        }
    }
    {
        println!("R850");
        let mut evo = Evolution::new(256, 850_f32, "cutting/r850.csv");
        let mut i = 0;
        for i in 0..50000 {
            evo.advance();
            if i % 1000 == 0 {
                let res = evo.get_best_result();
                println!("score {} in iter {}", res, i);
            }
        }
        println!("Score {} for r850 in 5000 iters", evo.get_best_result());
    }
}

struct Evolution {
    pub population: Vec<Chromosome>,
    pub task_data: TaskData,
}
impl Evolution {
    pub fn new<P: AsRef<std::path::Path>>(n: i32, r: f32, path: P) -> Evolution {
        assert!(n % 2 == 0);
        assert!(n >= 4);
        let pop: Vec<Chromosome> = (0..n).into_iter().map(|_| Chromosome::new()).collect();
        let task_data = TaskData::from_file(r, path);
        Evolution {
            population: pop,
            task_data,
        }
    }

    fn get_best_result(&self) -> f32 {
        self.population
            .iter()
            .map(|p| p.score())
            .fold(0.0, |a, b| a.max(b))
    }

    pub fn advance(&mut self) {
        self.advance_crossover();
        self.advance_mutation();
    }
    fn advance_crossover(&mut self) {
        let pop_count = self.population.len();
        let mut population = self
            .population
            .iter()
            .map(|c| (c.score(), c))
            .collect::<Vec<(f32, &Chromosome)>>();
        population.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let population: Vec<Chromosome> = population.into_iter().map(|t| t.1.to_owned()).collect();
        let best = &population[0..pop_count / 2];
        let mut children = Vec::with_capacity(pop_count);
        // lose bottom half, replace by crossover
        for pop_idx in (0..pop_count / 2).step_by(2) {
            let lparent = &best[pop_idx];
            let rparent = &best[pop_idx + 1];

            let new_children = Chromosome::crossover(lparent, rparent, &self.task_data);
            children.push((new_children.0).to_owned());
            children.push((new_children.1).to_owned());
            children.push(lparent.to_owned());
            children.push(rparent.to_owned());
        }
        self.population = children;
    }
    fn advance_mutation(&mut self) {
        self.population
            .par_iter_mut()
            .for_each(|mut c| c.mutate(&self.task_data));
    }
}

#[derive(Clone)]
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
    pub fn score(&self) -> f32 {
        self.rects.iter().map(|r| r.value).sum()
    }

    // viable_rects - sorted ascending according to value
    pub fn mutate(&mut self, task_data: &TaskData) {
        let circle = &task_data.circle;
        // mutate existing
        {
            let mut toremove: Vec<usize> = Vec::new();
            let rng = &mut rand::thread_rng();
            let probas = Uniform::new(0.0, 1.0).sample_iter(rng);
            let normal = Normal::new(0.0, 10.0).unwrap();
            'iter_rect: for (i, prob) in zip(0..self.rects.len(), probas) {
                if prob < self.rects[i].mut_prob {
                    let viable_overlaps: Vec<&Rect> = self
                        .rects
                        .iter()
                        .filter(|r| {
                            (self.rects[i].center.x - r.center.x).abs() <= task_data.max_width
                                && (self.rects[i].center.y - r.center.y).abs()
                                    <= task_data.max_height
                        })
                        .collect();
                    let mut rects_s: Vec<usize> = (0..task_data.rects.len()).collect();
                    rects_s.shuffle(&mut rand::thread_rng());
                    'find_viable: for viable_rect_idx in rects_s {
                        let viable_rect = &task_data.rects[viable_rect_idx];
                        let dx = normal.sample(&mut rand::thread_rng());
                        let dy = normal.sample(&mut rand::thread_rng());
                        let new_rect = Rect::new(
                            &self.rects[i].center.add_x(dx).add_y(dy),
                            viable_rect.height,
                            viable_rect.width,
                            viable_rect.value,
                            task_data.min_value,
                            task_data.max_value,
                        );
                        if (task_data.circle.contains(&new_rect)
                            && (self.rects[i].covers(viable_rect)
                                || !viable_overlaps.iter().any(|r| r.overlaps(&new_rect))))
                        {
                            self.rects[i] = new_rect;
                            continue 'iter_rect;
                        }
                    }
                    toremove.push(i);
                }
            }
            for i in toremove.into_iter().rev() {
                self.rects.remove(i);
            }
        }
        //  attempt n insertions
        {
            let new_rect_attempts = 10;
            let rng = &mut rand::thread_rng();
            let mut sampler = Uniform::new(-circle.radius, circle.radius).sample_iter(rng);
            for _ in 0..new_rect_attempts {
                let center = Pos {
                    x: sampler.next().unwrap(),
                    y: sampler.next().unwrap(),
                };
                if !circle.contains_point(&center) {
                    continue;
                }
                let viable_overlaps: Vec<&Rect> = self
                    .rects
                    .iter()
                    .filter(|r| {
                        (center.x - r.center.x).abs() <= task_data.max_width
                            && (center.y - r.center.y).abs() <= task_data.max_height
                    })
                    .collect();
                let mut rects_s: Vec<usize> = (0..task_data.rects.len()).collect();
                rects_s.shuffle(&mut rand::thread_rng());
                'place_best: for viable_rect_idx in rects_s {
                    let viable_rect = &task_data.rects[viable_rect_idx];
                    let new_rect = Rect::new(
                        &center,
                        viable_rect.height,
                        viable_rect.width,
                        viable_rect.value,
                        task_data.min_value,
                        task_data.max_value,
                    );
                    if !viable_overlaps.iter().any(|r| r.overlaps(&new_rect)) {
                        self.rects.push(new_rect);
                        break 'place_best;
                    }
                }
            }
        }
    }
    pub fn crossover(
        this: &Chromosome,
        other: &Chromosome,
        task_data: &TaskData,
    ) -> (Chromosome, Chromosome) {
        let radius = task_data.circle.radius;
        let slice_distr = Normal::new(0.0, radius / 2.0).unwrap();
        let self_rects_len = this.rects.len();
        let other_rects_len = other.rects.len();
        let slice_x = slice_distr
            .sample(&mut rand::thread_rng())
            .clamp(-radius, radius);

        let self_without_line = this
            .clone()
            .rects
            .into_iter()
            .filter(|r| !r.contains_vert_line(slice_x));
        let other_without_line = other
            .clone()
            .rects
            .into_iter()
            .filter(|r| !r.contains_vert_line(slice_x));

        let mut self_left = Vec::with_capacity(self_rects_len / 2);
        let mut self_right = Vec::with_capacity(self_rects_len / 2);
        for r in self_without_line {
            if (r.center.x <= slice_x) {
                self_left.push(r)
            } else {
                self_right.push(r)
            }
        }
        let mut other_left = Vec::with_capacity(other_rects_len / 2);
        let mut other_right = Vec::with_capacity(other_rects_len / 2);
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
