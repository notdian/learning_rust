use super::Sorter;

// Find the smallest element in the list , put it in the front, then do the same for the remainder
// Uses no extra memory
pub struct SelectionSort {
    use_iterator: bool,
}

impl Sorter for SelectionSort {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 0..slice.len() {
            let mut smallest_in_rest;
            if self.use_iterator {
                smallest_in_rest = slice[unsorted..]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, v)| v)
                    .map(|(i, _)| unsorted + i) // bc smallest_in_rest ^ is index in slice[unsorted..] we need to add unsorted
                    .expect("slice non-empty");
            } else {
                smallest_in_rest = unsorted;
                for possibly_smaller in (unsorted + 1)..slice.len() {
                    if slice[possibly_smaller] < slice[smallest_in_rest] {
                        smallest_in_rest = possibly_smaller;
                    }
                }
            }
            slice.swap(unsorted, smallest_in_rest)
        }
    }
}

#[test]
fn selection_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    SelectionSort {
        use_iterator: false,
    }
    .sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}

#[test]
fn selection_iter_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    SelectionSort { use_iterator: true }.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}
