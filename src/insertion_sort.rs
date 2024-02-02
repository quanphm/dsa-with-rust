pub fn insertion_sort<T>(collection: &mut [T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let len = collection.len();

    for i in 1..len {
        let mut j = i;
        while j > 0 && collection[j - 1] > collection[j] {
            collection.swap(j - 1, j);
            j -= 1;
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
        assert_eq!(vec![1, 2, 3, 4, 5], insertion_sort(&mut arr));

        let mut arr = vec![2, 1, 1, 0, 3];
        assert_eq!(vec![0, 1, 1, 2, 3], insertion_sort(&mut arr));

        let mut arr = ['b', 'd', 'a', 'c'];
        assert_eq!(vec!['a', 'b', 'c', 'd'], insertion_sort(&mut arr[..]));
    }
}
