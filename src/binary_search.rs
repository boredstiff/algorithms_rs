#[allow(dead_code)]
pub fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let mut low = 0;
    let mut high = haystack.len();
    while low < high {
        let midpoint = low + (high - low) / 2;
        let value = haystack[midpoint];

        if value == needle {
            return true;
        } else if value > needle {
            high = midpoint
        } else {
            low = midpoint + 1
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let haystack: [i32; 11] = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_search(&haystack, 69), true);
        assert_eq!(binary_search(&haystack, 1336), false);
        assert_eq!(binary_search(&haystack, 69420), true);
        assert_eq!(binary_search(&haystack, 69421), false);
        assert_eq!(binary_search(&haystack, 1), true);
        assert_eq!(binary_search(&haystack, 0), false);
    }
}
