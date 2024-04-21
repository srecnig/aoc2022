// code starts here

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
}

struct Tree {
    x: usize,
    y: usize,
    height: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
