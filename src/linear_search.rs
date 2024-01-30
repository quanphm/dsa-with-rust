pub fn linear_search<T>(haystack: &[T], needle: &T) -> Option<usize>
where
    T: PartialOrd,
{
    for i in 0..haystack.len() {
        if &haystack[i] == needle {
            return Some(i);
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
        assert_eq!(Some(5), linear_search(&haystack, &target));
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        let haystack = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let target = 10;
        b.iter(|| linear_search(&haystack, &target));
    }
}
