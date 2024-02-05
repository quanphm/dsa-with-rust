pub fn bubble_sort<T>(collection: &mut [T])
where
    T: PartialOrd + Clone,
{
    let length = collection.len();
    for i in 0..(length - 1) {
        for j in 0..(length - i - 1) {
            if collection[j] > collection[j + 1] {
                collection.swap(j, j + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let mut arr = [2, 1, 4, 3, 5];
        bubble_sort(&mut arr);
        assert_eq!([1, 2, 3, 4, 5], arr);

        let mut arr = vec![2, 1, 1, 0, 3];
        bubble_sort(&mut arr);
        assert_eq!(vec![0, 1, 1, 2, 3], arr);

        let mut arr = ['b', 'd', 'a', 'c'];
        let mut slice = &mut arr[..];
        bubble_sort(&mut slice);
        assert_eq!(['a', 'b', 'c', 'd'], slice);
    }
}
