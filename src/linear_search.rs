#[allow(dead_code)]
pub fn linear_search(haystack: &[i32], needle: i32) -> bool {
    if haystack.contains(&needle) {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let haystack: [i32; 11] = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(linear_search(&haystack, 69), true);
        assert_eq!(linear_search(&haystack, 1336), false);
        assert_eq!(linear_search(&haystack, 69420), true);
        assert_eq!(linear_search(&haystack, 69421), false);
        assert_eq!(linear_search(&haystack, 1), true);
        assert_eq!(linear_search(&haystack, 0), false);
    }
}
