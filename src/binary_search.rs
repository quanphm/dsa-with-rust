pub fn binary_search<T>(haystack: &[T], needle: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match &haystack[mid] {
            m if m > needle => high = mid,
            m if m < needle => low = mid + 1,
            _ => return Some(mid),
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
        let target = 42;
        assert_eq!(Some(13), binary_search(&haystack, &target));
    }
}
