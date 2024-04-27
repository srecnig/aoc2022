// code starts here

use std::vec;

pub fn grow_forest(lines: Vec<&str>) -> Forest {
    let y_size = lines.len();
    let x_size = lines[0].len();
    let mut forest = Forest {
        x_size,
        y_size,
        trees: Vec::new(),
    };
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let height: i32 = char.to_digit(10).unwrap() as i32;
            forest.plant_tree(x, y, height);
        }
    }
    forest
}

struct Forest {
    x_size: usize,
    y_size: usize,
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn plant_tree(&mut self, x: usize, y: usize, height: i32) {
        match self.trees.get_mut(y) {
            Some(row) => row.insert(x, Tree { x, y, height }),
            None => self.trees.insert(y, vec![Tree { x, y, height }]),
        }
    }

    fn tree_at(&self, x: usize, y: usize) -> &Tree {
        self.trees.get(y).unwrap().get(x).unwrap()
    }

    pub fn visible_tree_count(&self) -> i32 {
        let mut left_to_right: Vec<&Tree> = vec![];
        for row in &self.trees[1..self.trees.len() - 1] {
            // first and last are always ignored
            let mut biggest_height = row[0].height;
            for tree in &row[1..row.len() - 1] {
                if tree.height > biggest_height {
                    left_to_right.push(tree);
                    biggest_height = tree.height;
                }
            }
        }
        println!("********");
        println!("{:?}", left_to_right);

        let mut right_to_left: Vec<&Tree> = vec![];
        for row in &self.trees[1..self.trees.len() - 1] {
            // first and last are always ignored
            let mut biggest_height = row.last().unwrap().height;
            for tree in row[1..row.len() - 1].iter().rev() {
                if tree.height > biggest_height {
                    right_to_left.push(tree);
                    biggest_height = tree.height;
                }
            }
        }
        println!("#######");
        println!("{:?}", right_to_left);

        // [Tree { x: 2, y: 1, height: 5 }, yes
        // Tree { x: 3, y: 2, height: 3 },
        // Tree { x: 1, y: 2, height: 5 }]

        // 30373
        // 25512
        // 65332
        // 33549
        // 35390

        // The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
        // The top-middle 5 is visible from the top and right. yes
        // The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
        // The left-middle 5 is visible, but only from the right. yes
        // The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
        // The right-middle 3 is visible from the right. yes
        // In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

        5
    }
}

#[derive(Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: i32,
    // Eq trait
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_visible_trees() {
        let forest = setup_forest();
        forest.visible_tree_count();
    }

    #[test]
    fn can_grow_forest() {
        let forest_description = r#"30373
25512
65332
33549
35390"#;
        let forest = grow_forest(forest_description.lines().collect());
        assert_eq!(3, forest.tree_at(0, 0).height);
        assert_eq!(7, forest.tree_at(3, 0).height);
        assert_eq!(3, forest.tree_at(4, 0).height);
        assert_eq!(0, forest.tree_at(4, 4).height);
        assert_eq!(9, forest.tree_at(3, 4).height);
        assert_eq!(3, forest.tree_at(0, 4).height);
    }

    #[test]
    fn can_plant_tree() {
        let mut forest = Forest {
            x_size: 3,
            y_size: 2,
            trees: Vec::new(),
        };
        forest.plant_tree(0, 0, 5);
        forest.plant_tree(1, 0, 4);
        forest.plant_tree(2, 0, 3);
        forest.plant_tree(0, 1, 2);
        forest.plant_tree(1, 1, 6);
        forest.plant_tree(2, 1, 8);
        assert_eq!(8, forest.tree_at(2, 1).height);
        assert_eq!(3, forest.tree_at(2, 0).height);
        assert_eq!(6, forest.tree_at(1, 1).height);
    }

    fn setup_forest() -> Forest {
        let forest_description = r#"30373
25512
65332
33549
35390"#;
        grow_forest(forest_description.lines().collect())
    }
}
