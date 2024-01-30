pub fn bubble_sort<T>(collection: &[T]) -> Vec<T>
where
    T: Clone + PartialOrd,
{
    let length = collection.len();
    let mut result: Vec<T> = collection.into();
    for i in 0..length - 1 {
        for j in 0..length - i - 1 {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1)
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let arr = [2, 1, 4, 3, 5];
        assert_eq!(vec![1, 2, 3, 4, 5], bubble_sort(&arr));

        let arr = [2, 1, 1, 0, 3];
        assert_eq!(vec![0, 1, 1, 2, 3], bubble_sort(&arr));

        let arr = ['b', 'd', 'a', 'c'];
        assert_eq!(vec!['a', 'b', 'c', 'd'], bubble_sort(&arr));
    }
}
