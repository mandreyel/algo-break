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
        if !seq.is_empty() {
            rquick_sort_impl(seq);
        }
    }

    fn rquick_sort_impl<T: PartialOrd>(seq: &mut [T]) {
        if seq.len() > 1 {
            let pivot = partition(seq);
            assert!(pivot < seq.len());
            rquick_sort_impl(&mut seq[..pivot]);
            rquick_sort_impl(&mut seq[pivot + 1..]);
        }
    }

    /// Partitions `seq` such that elements smaller than a pivot value (the
    /// first element of `seq`) are placed on the left side, while elements
    /// equal or larger than pivot are placed on the right side.
    // FIXME: can't handle duplicates
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

            // If indices met each other or they're on the same element, quit
            // (this is to avoid infinite loops when `seq` has e.g. two of the
            // same elements, in which case the indices would never meet).
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

    #[test]
    fn insertion_sort() {
        let mut array = [3, 9, 2, 1];
        sort::insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 9]);
    }

    #[test]
    fn rquick_sort() {
        let mut array = [5, 10, 3, 3, 9, 2, 1];
        sort::rquick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 3, 5, 9, 10]);

        let mut array = [5, 10, 3, 9, 2, 1];
        sort::rquick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 5, 9, 10]);
    }
}
