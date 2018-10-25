pub mod sort {
    pub fn insertion_sort<T: PartialOrd>(seq: &mut [T]) {
        let mut i = 1;
        while i < seq.len() {
            let mut k = i;
            while k > 0 && seq[k - 1] > seq[k] {
                seq.swap(k, k - 1);
                k -= 1;
            }
            i += 1;
        }
    }

    pub fn rquick_sort<T: PartialOrd + Clone>(seq: &mut [T]) {
        if seq.len() > 1 {
            let pivot = partition(seq);
            assert!(pivot < seq.len());
            rquick_sort(&mut seq[..pivot]);
            rquick_sort(&mut seq[pivot + 1..]);
        }
    }

    /// Partitions `seq` such that elements smaller than a pivot value (the
    /// first element of `seq`) are placed on the left side, while elements
    /// equal or larger than pivot are placed on the right side. This is the
    /// Hoare partition scheme.
    // FIXME duplicate elements aren't handled correctly
    fn partition<T: PartialOrd>(seq: &mut [T]) -> usize {
        assert!(!seq.is_empty());
        let pivot = 0;
        let mut lo = 0;
        let mut hi = seq.len() - 1;
        loop {
            while seq[lo] < seq[pivot] {
                lo += 1;
            }

            while seq[hi] > seq[pivot] {
                hi -= 1;
            }

            // FIXME
            if lo >= hi || (!(seq[lo] > seq[hi]) && !(seq[lo] < seq[hi])) {
                return hi;
            }

            seq.swap(lo, hi);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sorter<Sorter: Fn(&mut [i32])>(sort: Sorter) {
        let mut array = [5, 10, 3, 3, 9, 2, 1];
        sort(&mut array);
        assert_eq!(array, [1, 2, 3, 3, 5, 9, 10]);

        let mut array = [5, 10, 3, 2, 3, 9, 2, 1];
        sort(&mut array);
        assert_eq!(array, [1, 2, 2, 3, 3, 5, 9, 10]);

        let mut array = [5, 10, 3, 9, 2, 1];
        sort(&mut array);
        assert_eq!(array, [1, 2, 3, 5, 9, 10]);

        let mut array = [5, 5, 5];
        sort(&mut array);
        assert_eq!(array, [5, 5, 5]);

        let mut array = [1];
        sort(&mut array);
        assert_eq!(array, [1]);

        let mut array: [i32; 0] = [];
        sort(&mut array);
        assert_eq!(array, []);
    }

    #[test]
    fn insertion_sort() {
        test_sorter(|mut array| sort::insertion_sort(&mut array));
    }

    #[test]
    fn rquick_sort() {
        test_sorter(|mut array| sort::rquick_sort(&mut array));
    }
}
