use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord,
    {
        if slice.len() < 2 {
            return;
        }
        let mut swapped = true;

        while swapped {
            // we reset swapped to false at beginning so we detect when now more swaps are happening
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubble_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    BubbleSort.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}
