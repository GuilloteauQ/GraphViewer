// pub mod svg;
extern crate rand;

use svg::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    /// Returns a new point
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    /// Returns a random point inside the rectangle n*m
    pub fn random_point(n: u32, m: u32) -> Point {
        let x: u32 = rand::random();
        let y: u32 = rand::random();
        Point::new(x % n, y % m)
    }

    /// Returns the squared distance between 2 points
    pub fn distance(&self, pt: &Point) -> u32 {
        let diff_x: i32 = self.x as i32 - pt.x as i32;
        let diff_y: i32 = self.y as i32 - pt.y as i32;
        (diff_x * diff_x + diff_y * diff_y) as u32
    }

    /// Compare if two points are diffrents
    pub fn is_same_as(&self, pt: &Point) -> bool {
        (self.x == pt.x && self.y == pt.y)
    }

    /// Returns true if the point is in the vector
    pub fn is_in(&self, vec: &Vec<Point>) -> bool {
        vec.iter().any(|pt| self.is_same_as(pt))
    }

    /// Returns the closest point and removes it from the vector
    pub fn closest_point(&self, vec: &mut Vec<Point>) -> Point {
        let index = vec.iter()
            .enumerate()
            .min_by_key(|&(_, pt)| self.distance(pt))
            .unwrap()
            .0;
        vec.swap_remove(index)
    }

    /// Draws a point (circle)
    pub fn draw_point(&self, svg_file: &mut Svg) {
        svg_file.circle(self.x, self.y, 3, "black".to_string());
    }

    /// Draws a line between two points
    pub fn draw_line(&self, pt: &Point, svg_file: &mut Svg) {
        svg_file.line(self.x, self.y, pt.x, pt.y, "blue".to_string());
    }

    /// Draws an animated line between two points
    pub fn draw_animated_line(&self, pt: &Point, svg_file: &mut Svg, begin: u32) {
        svg_file.line_animated(self.x, self.y, pt.x, pt.y, "blue".to_string(), begin);
    }
}

/// Returns a vector of random points
pub fn random_vec(size_x: u32, size_y: u32, n: u32) -> Vec<Point> {
    let mut vec: Vec<Point> = Vec::new();
    let mut index: u32 = 0;
    while index < n {
        let pt: Point = Point::random_point(size_x, size_y);
        if !pt.is_in(&vec) {
            vec.push(pt);
            index += 1;
        }
    }
    vec
}
