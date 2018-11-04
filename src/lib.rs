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
        }
    }

    /// Returns a graph with the matrix set
    pub fn new_with_points(points: &Vec<Point>) -> Self {
        let n = points.len();
        let mut graph = Graph::new(n);
        // --- Computing the distances between all the points ---
        for (index, pt) in points.iter().enumerate() {
            for j in index + 1..n {
                graph.add_point(index, j, pt.distance(&points[j]));
            }
        }

        graph
    }

    /// Add a point to the matrix
    pub fn add_point(&mut self, pt1: usize, pt2: usize, dist: u32) {
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
            result.add_point(min_pt, from_point, self.matrix[min_pt][from_point]);
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
    pub fn draw_graph_svg(&self, vec_points: &Vec<Point>, svg_file: &mut Svg) {
        // The header
        svg_file.header();
        // The background
        svg_file.rectangle(0, 0,svg_file.h, svg_file.w, "white".to_string());
        for (index, pt) in vec_points.iter().enumerate() {
            pt.draw_point(svg_file);
            for (i, _) in self.matrix[index].iter().enumerate().filter(|&(i, _)| self.matrix[index][i] != u32::max_value()) {
                pt.draw_line(&vec_points[i], svg_file);
            }
        }
        // The footer
        svg_file.footer();
    }
}


#[cfg(test)]
mod tests {
    use svg::*;
    use points::*;
    use super::Graph;

    #[test]
    fn random_graph() {
        let w = 800;
        let h = 800;
        let n = 50;
        let mut svg_file = Svg::new("out.svg".to_string(), h, w);
        let vec: Vec<Point> = random_vec(w, h, n);
        let graph: Graph = Graph::new_with_points(&vec);
        graph.draw_graph_svg(&vec, &mut svg_file);
        let mut svg_tree = Svg::new("spanning_tree.svg".to_string(), h, w);
        let tree = graph.spanning_tree(0);
        tree.draw_graph_svg(&vec, &mut svg_tree);
    }
}
