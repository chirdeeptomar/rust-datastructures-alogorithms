//! Algorithm implementated in Rust

pub mod sorting {
    /// Complexity: O(n^2)
    /// not efficient, specially on large arrays
    pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
        for p in 0..v.len() {
            let mut is_sorted = true;

            for i in 0..(v.len() - 1) - p {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    is_sorted = false;
                }
            }

            if is_sorted {
                return;
            }
        }
    }
}
