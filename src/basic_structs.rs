use std::ops::Sub;

use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn magn(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn quadrant(&self) -> Quadrant {
        let top = self.y >= 0.0;
        let right = self.x >= 0.0;
        match (top, right) {
            (true, true) => Quadrant::First,
            (true, false) => Quadrant::Second,
            (false, true) => Quadrant::Third,
            (false, false) => Quadrant::Fourth,
        }
    }

    pub fn add_x(mut self, x: f32) -> Pos {
        self.x += x;
        self
    }
    pub fn add_y(mut self, y: f32) -> Pos {
        self.y += y;
        self
    }
}

impl Add for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Pos {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(mut self, rhs: Pos) -> Pos {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl Sub for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Pos {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(mut self, rhs: Pos) -> Pos {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

#[derive(PartialEq)]
pub enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl Quadrant {
    pub(crate) fn opposite(&self) -> Quadrant {
        match self {
            Quadrant::First => Quadrant::Third,
            Quadrant::Second => Quadrant::Fourth,
            Quadrant::Third => Quadrant::First,
            Quadrant::Fourth => Quadrant::Second,
        }
    }
}

pub struct Circle {
    // 0,0 assumed center
    pub radius: f32,
}

pub struct Rect {
    pub center: Pos,
    pub area: f32,
    pub value: f32,
    pub avgvalue: f32,
    pub quadrant: Quadrant,
    pub height: f32,
    pub width: f32,
    pub topleft: Pos,
    pub topright: Pos,
    pub bottomleft: Pos,
    pub bottomright: Pos,
    pub mut_prob: f32,
}

impl Rect {
    pub fn new(
        center: &Pos,
        height: f32,
        width: f32,
        value: f32,
        minvalue: f32,
        maxvalue: f32,
    ) -> Rect {
        let area = width * height;
        let halfwidth = width / 2.0;
        let halfheight = height / 2.0;
        let norm_val = (value - minvalue) / (maxvalue - minvalue);
        // [0; 0.6] probabilities
        let mut_prob = ((1.0 - norm_val).exp() / std::f32::consts::E) * 0.6;
        Rect {
            center: *center,
            area,
            value,
            avgvalue: area / value,
            quadrant: center.quadrant(),
            height,
            width,
            topleft: center.add_x(halfwidth).add_y(halfheight),
            topright: center.add_x(halfwidth).add_y(halfheight),
            bottomleft: center.add_x(halfwidth).add_y(halfheight),
            bottomright: center.add_x(halfwidth).add_y(halfheight),
            mut_prob,
        }
    }
    pub fn overlaps(&self, other: &Rect) -> bool {
        let relation = (other.center - self.center).quadrant();
        let corner_vert_relation = match relation {
            Quadrant::First => (self.topright - other.bottomleft).quadrant(),
            Quadrant::Second => (self.topleft - other.bottomright).quadrant(),
            Quadrant::Third => (self.bottomleft - other.topright).quadrant(),
            Quadrant::Fourth => (self.bottomright - other.topleft).quadrant(),
        };
        relation == corner_vert_relation.opposite()
    }
    pub fn covers(&self, other: &ViableRect) -> bool {
        // assuming same center
        self.height >= other.height && self.width >= other.width
    }

    pub fn contains_vert_line(&self, line_x: f32) -> bool {
        self.center.x - self.width / 2.0 <= line_x && line_x <= self.center.x + self.width / 2.0
    }
}

impl Circle {
    pub fn contains(&self, rect: &Rect) -> bool {
        let vert = match rect.quadrant {
            Quadrant::First => rect.topright,
            Quadrant::Second => rect.topleft,
            Quadrant::Third => rect.bottomleft,
            Quadrant::Fourth => rect.bottomright,
        };
        vert.magn() <= self.radius
    }
    pub fn contains_point(&self, pos: &Pos) -> bool {
        pos.magn() <= self.radius
    }
}

pub struct ViableRect {
    pub height: f32,
    pub width: f32,
    pub value: f32,
}

pub struct ViableRectData<'a> {
    pub min_value: f32,
    pub max_value: f32,
    pub max_width: f32,
    pub max_height: f32,
    pub rects: &'a Vec<ViableRect>,
}
