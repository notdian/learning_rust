use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord,
    {
        // [Sorted | not Sorted]

        for unsorted in 1..slice.len() {
            // slice[unsorted...] is not sorted
            // take slice[unsorted] and place in correct position in slice[..=unsorted]
            // [1 3 4 | 2 ]
            // [1 3 2 4 | ] <- start swapping
            // [1 2 3 4 |  ]

            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,  // found loc
                    Err(i) => i, // didn't find any el in correct loc
                };
                slice[i..=unsorted].rotate_right(1)
            }
        }
    }
}

#[test]
fn insertion_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    InsertionSort { smart: false }.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}

#[test]
fn insertion_bin_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    InsertionSort { smart: true }.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}
