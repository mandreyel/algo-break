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

    // FIXME: can't handle duplicates
    pub fn rquick_sort<T: PartialOrd + Clone>(seq: &mut [T]) {
        if !seq.is_empty() {
            let lo = 0;
            let hi = seq.len() - 1;
            rquick_sort_impl(seq, lo, hi);
        }
    }

    fn rquick_sort_impl<T: PartialOrd>(seq: &mut [T], lo: usize, hi: usize) {
        if lo < hi {
            let pivot = partition(seq, lo, hi);
            assert!(pivot < seq.len());
            rquick_sort_impl(seq, lo, pivot);
            rquick_sort_impl(seq, pivot + 1, hi);
        }
    }

    /// Partitions `seq` such that elements smaller than a randomly chosen
    /// pivot value are placed on the left side, while elements equal or
    /// larger than pivot are placed on the right side.
    fn partition<T: PartialOrd>(seq: &mut [T], mut lo: usize, mut hi: usize) -> usize {
        assert!(!seq.is_empty());
        assert!(lo < seq.len());
        assert!(hi < seq.len());
        let pivot = lo;
        loop {
            while seq[lo] < seq[pivot] {
                lo += 1;
            }

            while seq[hi] > seq[pivot] {
                hi -= 1;
            }

            if lo >= hi {
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
        let mut array = [5, 10, 3, 9, 2, 1];
        sort::rquick_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 5, 9, 10]);
    }
}
