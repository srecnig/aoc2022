use std::collections::{HashMap, VecDeque};

pub fn detect_marker(signal: &str, marker_size: usize) -> usize {
    let offset = marker_size - 1;
    let mut window: VecDeque<char> = signal.chars().take(offset).collect();
    for (i, ch) in signal.chars().enumerate().skip(offset) {
        window.push_back(ch);
        if !has_duplicate(&window) {
            return i;
        }
        window.pop_front();
    }
    // should never happen
    return usize::MAX;
}

fn has_duplicate(window: &VecDeque<char>) -> bool {
    let mut counts = HashMap::new();
    for ch in window {
        *counts.entry(ch).or_insert(0) += 1;
    }
    !counts.values().all(|&count| count == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_signal() {
        let marker = detect_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(6, marker);
        let marker = detect_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(10, marker);
        let marker = detect_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14);
        assert_eq!(28, marker);
    }

    #[test]
    fn can_handle_string_without_duplicates() {
        let marker = detect_marker("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 4);
        assert_eq!(usize::MAX, marker);
    }

    #[test]
    fn can_check_for_duplicates() {
        let with_duplicate = VecDeque::from(['a', 't', 'm', 'm']);
        let without_duplicate = VecDeque::from(['h', 'a', 's', 'i']);
        assert!(has_duplicate(&with_duplicate));
        assert!(!has_duplicate(&without_duplicate));
    }
}
