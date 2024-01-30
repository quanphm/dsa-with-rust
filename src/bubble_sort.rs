pub fn bubble_sort<T>(collection: &mut [T]) -> Vec<T>
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
    collection.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array() {
        let mut arr = [2, 1, 4, 3, 5];
        assert_eq!(vec![1, 2, 3, 4, 5], bubble_sort(&mut arr));
    }
}
