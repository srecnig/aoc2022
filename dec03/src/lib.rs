use std::collections::HashSet;

pub fn inspect_rucksack(contents: &str) -> i32 {
    let half_index = contents.len() / 2;
    let front = &contents[..half_index];
    let back = &contents[half_index..];
    let shared_item = find_shared_item(front, back).unwrap();
    item_priority(shared_item).unwrap()
}

pub fn inspect_rucksack_group(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> i32 {
    let shared1 = find_shared_items(rucksack1, rucksack2);
    let shared = find_shared_items(&shared1, rucksack3);
    let item = shared.chars().next().unwrap();
    item_priority(item).unwrap()
}

fn find_shared_item(front: &str, back: &str) -> Option<char> {
    for c in front.chars() {
        if back.contains(c) {
            return Some(c);
        }
    }
    None
}

fn find_shared_items(string1: &str, string2: &str) -> String {
    let mut shared_items = HashSet::new();
    for c in string1.chars() {
        if string2.contains(c) {
            shared_items.insert(c);
        }
    }
    shared_items.into_iter().collect()
}

fn item_priority(c: char) -> Option<i32> {
    match c {
        'a'..='z' => Some(c as i32 - 'a' as i32 + 1),
        'A'..='Z' => Some(c as i32 - 'A' as i32 + 27),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_priority_from_rucksack_contents() {
        assert_eq!(16, inspect_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(38, inspect_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!(42, inspect_rucksack("PmmdzqPrVvPwwTWBwg"));
        assert_eq!(22, inspect_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!(20, inspect_rucksack("ttgJtRGJQctTZtZT"));
        assert_eq!(19, inspect_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn can_get_priority_from_rucksack_groups() {
        let priority1 = inspect_rucksack_group(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );
        assert_eq!(18, priority1);
    }

    #[test]
    fn can_find_shared_item() {
        assert_eq!(
            'p',
            find_shared_item("vJrwpWtwJgWr", "hcsFMMfFFhFp").unwrap(),
        );
        assert_eq!(
            'L',
            find_shared_item("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL").unwrap(),
        );
    }

    #[test]
    fn can_find_shared_items() {
        let string1 = "bucuaYna";
        let string2 = "abmmCc";
        let shared_items = find_shared_items(string1, string2);
        // actually shared chars
        for c in shared_items.chars() {
            assert!(string1.contains(c));
            assert!(string2.contains(c));
        }
    }

    #[test]
    fn can_get_item_priority() {
        assert_eq!(1, item_priority('a').unwrap());
        assert_eq!(26, item_priority('z').unwrap());
        assert_eq!(27, item_priority('A').unwrap());
        assert_eq!(52, item_priority('Z').unwrap());
    }
}
