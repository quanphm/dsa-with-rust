pub fn insertion_sort<T: PartialOrd>(collection: &mut [T]) {
    let len = collection.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && collection[j - 1] > collection[j] {
            collection.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let mut arr = [2, 1, 4, 3, 5];
        insertion_sort(&mut arr);
        assert_eq!([1, 2, 3, 4, 5], arr);

        let mut arr = vec![2, 1, 1, 0, 3];
        insertion_sort(&mut arr);
        assert_eq!(vec![0, 1, 1, 2, 3], arr);

        let mut arr = ['b', 'd', 'a', 'c'];
        let mut slice = &mut arr[0..3];
        insertion_sort(&mut slice);
        assert_eq!(['a', 'b', 'd'], slice);
    }
}
