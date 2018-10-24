pub mod sort {
    pub fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
        let mut i = 1;
        while i < array.len() {
            let mut k = i;
            while k > 0 && array[k - 1] > array[k] {
                array.swap(k, k - 1);
                k -= 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort() {
        let mut array = [3, 9, 2, 1];
        sort::insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 9]);
    }
}
