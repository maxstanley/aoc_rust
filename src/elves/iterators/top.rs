use binary_heap_plus::{BinaryHeap, MinComparator};

/// Returns the Top N elements in an iterator.
/// https://www.youtube.com/watch?v=udHjmno-tfA
pub trait Top<T> {
    /// Returns the Top N elements in an iterator.
    ///
    /// If N elements are not available, None is returned.
    fn top<const N: usize>(self) -> Option<Vec<T>>;
}

impl<T: Ord, U: Iterator<Item = T>> Top<T> for U {
    fn top<const N: usize>(self) -> Option<Vec<T>> {
        let mut heap = BinaryHeap::<T, MinComparator>::from_vec(Vec::with_capacity(N));

        self.for_each(|v| {
            heap.push(v);

            if heap.len() > 3 {
                heap.pop();
            }
        });

        if heap.len() == 3 {
            Some(heap.into_sorted_vec())
        } else {
            None
        }
    }
}
