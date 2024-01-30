pub fn binary_search<T>(haystack: &[T], needle: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut low = 0_usize;
    let mut high = haystack.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        match &haystack[mid] {
            m if m > needle => high = mid - 1,
            m if m < needle => low = mid + 1,
            _ => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    #[test]
    fn should_return_correct_index() {
        let haystack = vec![10, 12, 13, 16, 18, 19, 20, 21, 22, 23, 24, 33, 35, 42, 47];
        let target = 19;
        assert_eq!(Some(5), binary_search(&haystack, &target));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let target = 10;
        b.iter(|| binary_search(&haystack, &target));
    }
}
