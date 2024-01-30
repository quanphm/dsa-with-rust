pub fn interpolation_search(haystack: &[i32], needle: &i32) -> Option<usize> {
    let mut low = 0;
    let mut high = haystack.len();
    let low_value = haystack[low];
    let high_value = haystack[high - 1];

    while low < high && *needle >= low_value && *needle <= high_value {
        let offset = (*needle - low_value) * (high - low) as i32 / (high_value - low_value);
        let pos = low + offset as usize;

        // use if-else
        // if &haystack[pos] < needle {
        //     low = pos + 1;
        // } else if &haystack[pos] > needle {
        //     high = pos;
        // } else {
        //     break Some(pos);
        // }

        // using pattern matching
        match haystack[pos] {
            v if &v < needle => low = pos + 1,
            v if &v > needle => high = pos,
            _ => return Some(pos),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_index() {
        let haystack = vec![10, 12, 13, 16, 18, 19, 20, 21, 22, 23, 24, 33, 35, 42, 47];
        assert_eq!(Some(9), interpolation_search(&haystack, &23));
        assert_eq!(Some(1), interpolation_search(&haystack, &12));
        assert_eq!(Some(13), interpolation_search(&haystack, &42));
        assert_eq!(None, interpolation_search(&haystack, &50));
    }
}
