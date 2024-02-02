pub fn selection_sort<T>(collection: &mut [T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let len = collection.len();
    for i in 0..len {
        let mut min = collection[i].to_owned();
        for j in i..len {
            if collection[j] < min {
                min = collection[j].to_owned();
                collection.swap(i, j);
            }
        }
    }

    collection.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let mut arr = [2, 1, 4, 3, 5];
        assert_eq!(vec![1, 2, 3, 4, 5], selection_sort(&mut arr));

        let mut arr = vec![2, 1, 1, 0, 3];
        assert_eq!(vec![0, 1, 1, 2, 3], selection_sort(&mut arr));

        let mut arr = ['b', 'd', 'a', 'c'];
        assert_eq!(vec!['a', 'b', 'c', 'd'], selection_sort(&mut arr[..]));
    }
}
