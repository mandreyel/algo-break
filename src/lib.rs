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

    pub fn rquick_sort<T: PartialOrd>(seq: &mut [T]) {
        if seq.len() > 1 {
            let pivot = hoare_partition(seq);
            assert!(pivot < seq.len());
            rquick_sort(&mut seq[..pivot]);
            rquick_sort(&mut seq[pivot + 1..]);
        }
    }

    /// Partitions `seq` such that elements smaller than a pivot value (the
    /// first element of `seq`) are placed to the left of the pivot, while
    /// elements equal or larger than pivot are placed to the right of the
    /// pivot. This is the Hoare partition scheme.
    fn hoare_partition<T: PartialOrd>(seq: &mut [T]) -> usize {
        assert!(!seq.is_empty());
        let mut pivot = 0;
        let mut lo = 0;
        let mut hi = seq.len() - 1;
        loop {
            while seq[lo] < seq[pivot] {
                lo += 1;
            }
            while seq[hi] > seq[pivot] {
                hi -= 1;
            }

            // If the two indices met, we're done.
            if lo >= hi {
                return hi;
            }

            // In case of duplicate elements it's possible that the pivot value
            // is not unique but the algorithm will not advance either indices
            // (because they're the same as pivot), so we must nudge one of the
            // indices forward to either arrive at a non-pivot value or meet the
            // other index.
            if seq[lo] == seq[pivot] && seq[hi] == seq[pivot] {
                lo += 1;
                continue;
            }

            seq.swap(lo, hi);

            // If either lo or hi was pivot, the pivot index needs to be updated.
            if pivot == lo {
                pivot = hi;
            } else if pivot == hi {
                pivot = lo;
            }
        }
    }

    fn lomuto_partition<T: PartialOrd>(seq: &mut [T]) -> usize {
        assert!(!seq.is_empty());
        let mut i = 0;
        let mut k = i;
        let pivot = seq.len() - 1;

        while k < seq.len() - 1 {
            if seq[k] < seq[pivot] {
                seq.swap(i, k);
                i += 1;
            }
            k += 1;
        }

        if *seq.last().unwrap() < seq[i] {
            seq.swap(i, pivot);
        }

        i
    }
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{self, Rng};
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

        let mut array = [5, 1, 5, 1, 5, 1, 5];
        sort(&mut array);
        assert_eq!(array, [1, 1, 1, 5, 5, 5, 5]);

        let mut array = [1];
        sort(&mut array);
        assert_eq!(array, [1]);

        let mut array: [i32; 0] = [];
        sort(&mut array);
        assert_eq!(array, []);

        // Taken from https://github.com/servo/rust-quicksort/blob/master/lib.rs
        let mut rng = rand::thread_rng();
        for _ in 0u32 .. 50000u32 {
            let len: usize = rng.gen();
            let mut v: Vec<i32> = rng
                .gen_iter::<i32>()
                .take((len % 32) + 1)
                .collect();
            sort(&mut v[..]);
            for i in 0 .. v.len() - 1 {
                assert!(v[i] <= v[i + 1])
            }
        }
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
