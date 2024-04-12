pub fn max_calories(calories: &Vec<Vec<i32>>) -> i32 {
    let sums: Vec<i32> = calories.iter().map(|c| c.into_iter().sum()).collect();
    sums.into_iter().max().unwrap_or(0)
}

pub fn max_calories_top_n(calories: &Vec<Vec<i32>>, n: usize) -> i32 {
    let mut sums: Vec<i32> = calories.iter().map(|c| c.into_iter().sum()).collect();
    sums.sort_unstable();
    sums.into_iter().rev().take(n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_max_calories() {
        let calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(24000, max_calories(&calories));
    }

    #[test]
    fn can_calculate_max_calories_top_n() {
        let calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(24000, max_calories_top_n(&calories, 1));
        assert_eq!(34000, max_calories_top_n(&calories, 2));
    }
}
