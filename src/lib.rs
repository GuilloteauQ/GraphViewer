pub mod svg;
pub mod points;

use svg::*;
use points::Point;

/// /////////////////////////
/// # The structure of Graph
///
/// A graph is represented by a matrix of adjacence
///

pub struct Graph {
    /// The number of points in the graph
    pub n: usize,
    /// The adjacence matrix of the graph
    pub matrix: Vec<Vec<u32>>,
    /// The vector of points in the graph
    pub points: Vec<Point>,
}

impl Graph {
    /// Returns a empty graph
    pub fn new(n: usize) -> Self {
        let mut matrix = Vec::new();
        for _ in 0..n {
            let sub_mat = (0..n).map(|_| u32::max_value()).collect();
            matrix.push(sub_mat);
        }
        Graph {
            n: n,
            matrix: matrix,
            points: Vec::new(),
        }
    }

    /// Returns a graph with the matrix set
    pub fn new_with_points(points: &Vec<Point>) -> Self {
        let n = points.len();
        let mut graph = Graph::new(n);
        // --- Computing the distances between all the points ---
        for (index, pt) in points.iter().enumerate() {
            for j in index + 1..n {
                graph.add_edge(index, j, pt.distance(&points[j]));
            }
        }
        for pt in points.iter() {
            graph.points.push(*pt);
        }

        graph
    }

    /// Add an edge in the matrix
    pub fn add_edge(&mut self, pt1: usize, pt2: usize, dist: u32) {
        self.matrix[pt1][pt2] = dist;
        self.matrix[pt2][pt1] = dist;
    }

    /// Returns the spanning tree from the root point using Prim's Algorithm
    pub fn spanning_tree(&self, root: usize) -> Self {
        let mut result = Graph::new(self.n);
        let mut set_of_points = Vec::new();
        set_of_points.push(root);

        // While we don't have a spanning tree
        while set_of_points.len() != self.n {
            let mut min = u32::max_value();
            let mut min_pt = 0;
            let mut from_point = 0;
            // For all the points currently in the spanning tree
            for pt in set_of_points.iter() {
                for next_point in 0..self.n {
                    // If the point is not in the spanning tree
                    if !set_of_points.iter().any(|point| point == &next_point) {
                        // If it is the closest from the spanning tree
                        if self.matrix[*pt][next_point] < min {
                            min = self.matrix[*pt][next_point];
                            min_pt = next_point;
                            from_point = *pt;
                        }
                    }
                }
            }
            // We add the closest point from the spanning tree to the tree
            set_of_points.push(min_pt);
            result.add_edge(min_pt, from_point, self.matrix[min_pt][from_point]);
        }
        for pt in self.points.iter() {
            result.points.push(*pt);
        }
        result
    }

    /// Returns the distance for the smallest path using the spanning tree
    /// ARG: a spanning tree
    pub fn path_in_spanning_tree(&self, root: usize, full_graph: &Graph) -> u32 {
        // The stack of the points to see
        let mut stack = Vec::new();
        // The distance of the final path
        let mut cumulated_distance = 0;
        let mut previous_point = root;
        // An arraty representing which point we have already seen
        let mut already_seen: Vec<bool> = (0..self.n).map(|_| false).collect();

        already_seen[root] = true;
        stack.push(root);
        // While there is some point to see
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            for pt in 0..self.n {
                // For all the neighbours of top that have not yet been seen
                if !already_seen[pt] && self.matrix[top][pt] != u32::max_value() {
                    // We add those point to be seen
                    stack.push(pt);
                    already_seen[pt] = true;
                }
            }
            cumulated_distance += full_graph.matrix[top][previous_point];
            previous_point = top;
        }
        cumulated_distance
    }


    /// Draws the graph
    pub fn draw_graph_svg(&self,  svg_file: &mut Svg) {
        // The header
        // svg_file.header();
        // The background
        svg_file.rectangle(0, 0,svg_file.h, svg_file.w, "white".to_string());
        for (index, pt) in self.points.iter().enumerate() {
            pt.draw_point(svg_file);
            for (i, _) in self.matrix[index].iter().enumerate().filter(|&(i, _)| self.matrix[index][i] != u32::max_value()) {
                pt.draw_line(&self.points[i], svg_file);
            }
        }
        // The footer
        // svg_file.footer();
    }

    /// Returns the spanning tree from the root point using Prim's Algorithm
    pub fn draw_animated_spanning_tree(&self, root: usize, svg_file: &mut Svg) {
        svg_file.rectangle(0, 0,svg_file.h, svg_file.w, "white".to_string());
        let mut set_of_points = Vec::new();
        set_of_points.push(root);

        for pt in self.points.iter() {
            pt.draw_point(svg_file);
        }

        // While we don't have a spanning tree
        while set_of_points.len() != self.n {
            let mut min = u32::max_value();
            let mut min_pt = 0;
            let mut from_point = 0;
            // For all the points currently in the spanning tree
            for pt in set_of_points.iter() {
                for next_point in 0..self.n {
                    // If the point is not in the spanning tree
                    if !set_of_points.iter().any(|point| point == &next_point) {
                        // If it is the closest from the spanning tree
                        if self.matrix[*pt][next_point] < min {
                            min = self.matrix[*pt][next_point];
                            min_pt = next_point;
                            from_point = *pt;
                        }
                    }
                }
            }
            // We add the closest point from the spanning tree to the tree
            set_of_points.push(min_pt);
            self.points[from_point].draw_animated_line(&self.points[min_pt], svg_file, (10000 * set_of_points.len() / self.n) as u32);
        }
    }


    /// Returns the distance for the smallest path using the spanning tree
    /// ARG: a spanning tree
    pub fn animated_path_in_spanning_tree(&self, root: usize, svg_file: &mut Svg) -> u32 {
        // The stack of the points to see
        let mut stack = Vec::new();
        // The distance of the final path
        let mut cumulated_distance = 0;
        let mut previous_point = root;
        // An arraty representing which point we have already seen
        let mut already_seen: Vec<bool> = (0..self.n).map(|_| false).collect();
        let mut seen_points = 1;

        already_seen[root] = true;
        stack.push(root);
        // While there is some point to see
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            for pt in 0..self.n {
                // For all the neighbours of top that have not yet been seen
                if !already_seen[pt] && self.matrix[top][pt] != u32::max_value() {
                    // We add those point to be seen
                    stack.push(pt);
                    already_seen[pt] = true;
                }
            }
            self.points[previous_point].draw_animated_line_red(&self.points[top], svg_file, (1 + (10000 * seen_points / self.n)) as u32);
            cumulated_distance += self.points[previous_point].distance(&self.points[top]);
            previous_point = top;
            seen_points += 1;
        }
        cumulated_distance
    }



}

pub fn greedy_path(mut vec: &mut Vec<Point>, svg_file: &mut Svg) -> u32 {
    let n = vec.len();
    svg_file.rectangle(0, 0,svg_file.h, svg_file.w, "white".to_string());
    for pt in vec.iter() {
        pt.draw_point(svg_file);
    }
    // --- Definition of the main variables ---
    let mut current: Point = vec.swap_remove(0); //Point::new(0, 0);
    let mut cumulated_distance: u32 = 0;
    // --- End of the Definition of the main variables ---
    while vec.len() > 0 {
        let closest: Point = current.closest_point(&mut vec);
        cumulated_distance += current.distance(&closest);
        current.draw_animated_line_red(&closest, svg_file, ((n - vec.len()) * 10000 / n) as u32);
        current = closest;
    }
    cumulated_distance
}




#[cfg(test)]
mod tests {
    use svg::*;
    use points::*;
    use super::*;

    #[test]
    fn random_graph() {
        // ----- Defining main variables -----
        let w = 800;
        let h = 800;
        let n = 50;
        let mut svg_file = Svg::new("out.svg".to_string(), h, w);
        let mut vec: Vec<Point> = random_vec(w, h, n);
        let graph: Graph = Graph::new_with_points(&vec);

        // ----- Drawing the full graph -----
        svg_file.header();
        graph.draw_graph_svg(&mut svg_file);
        svg_file.footer();

        // ----- Drawing the spanning tree -----
        let mut svg_tree = Svg::new("spanning_tree.svg".to_string(), h, w);
        svg_tree.header();
        let tree = graph.spanning_tree(0);
        tree.draw_graph_svg(&mut svg_tree);
        svg_tree.footer();

        // ---- Drawing the animated spanning tree -----
        let mut svg_tree_animated = Svg::new("animated_spanning_tree.svg".to_string(), h, w);
        svg_tree_animated.header();
        graph.draw_animated_spanning_tree(0, &mut svg_tree_animated);
        svg_tree_animated.footer();

        // ----- Drawing the path in the spanning tree -----
        let mut svg_path_tree = Svg::new("path_spanning_tree.svg".to_string(), h, w);
        svg_path_tree.header();
        let tree = graph.spanning_tree(0);
        tree.draw_graph_svg(&mut svg_path_tree);
        let _ = tree.animated_path_in_spanning_tree(0, &mut svg_path_tree);
        svg_path_tree.footer();

        // ----- Drawing the greedy path in the graph -----
        let mut svg_path_greedy = Svg::new("path_greedy.svg".to_string(), h, w);
        svg_path_greedy.header();
        let _ = greedy_path(&mut vec, &mut svg_path_greedy);
        svg_path_greedy.footer();
    }
}

