pub mod sorting {
    pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
        let mut is_sorted = false;
        let mut last_unsorted = v.len() - 1;
        while !is_sorted {
            is_sorted = true;
            for i in 0..last_unsorted {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                    is_sorted = false;
                }
            }
            last_unsorted = last_unsorted - 1;
        }
    }
}
