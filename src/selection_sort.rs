pub fn selection_sort<T>(collection: &mut [T])
where
    T: PartialOrd + Clone,
{
    let len = collection.len();
    for i in 0..len {
        let mut min = i;
        for j in i..len {
            if collection[j] < collection[min] {
                min = j;
            }
        }
        collection.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let mut arr = [2, 1, 4, 3, 5];
        selection_sort(&mut arr);
        assert_eq!([1, 2, 3, 4, 5], arr);

        let mut arr = vec![2, 1, 1, 0, 3];
        selection_sort(&mut arr);
        assert_eq!(vec![0, 1, 1, 2, 3], arr);

        let mut arr = ['b', 'd', 'a', 'c'];
        let mut slice = &mut arr[..];
        selection_sort(&mut slice);
        assert_eq!(['a', 'b', 'c', 'd'], slice);
    }
}
