pub fn ranges_containment(ranges: &str) -> bool {
    let ranges: Vec<Range> = ranges.split(',').map(|r| build_range(r)).collect();
    ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0])
}

fn build_range(range: &str) -> Range {
    let borders: Vec<i32> = range.split('-').map(|num| num.parse().unwrap()).collect();
    Range {
        from: borders[0],
        to: borders[1],
    }
}

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_check_if_ranges_containment_is_present() {
        assert!(ranges_containment("6-6,4-6"));
        assert!(!ranges_containment("1-6,4-7"))
    }

    #[test]
    fn can_build_range() {
        let range = build_range("5-8");
        assert_eq!(5, range.from);
        assert_eq!(8, range.to);
    }

    #[test]
    fn can_check_for_containment() {
        let range1 = Range { from: 1, to: 3 };
        let range2 = Range { from: 4, to: 5 };
        assert!(!range1.contains(&range2));
        assert!(!range2.contains(&range1));

        let range1 = Range { from: 1, to: 5 };
        let range2 = Range { from: 3, to: 7 };
        assert!(!range1.contains(&range2));
        assert!(!range2.contains(&range1));

        let range1 = Range { from: 1, to: 7 };
        let range2 = Range { from: 2, to: 5 };
        assert!(range1.contains(&range2));
        assert!(!range2.contains(&range1));

        let range1 = Range { from: 4, to: 7 };
        let range2 = Range { from: 4, to: 7 };
        assert!(range1.contains(&range2));
        assert!(range2.contains(&range1));
    }
}
