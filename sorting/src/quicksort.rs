use std::vec;

use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    let (pivot, rest) = slice.split_first_mut().expect("slice is not-empty");

    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // account for pivot at 0
    let left = left + 1;

    // place the pivot in the final position
    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);

    assert!(left.last() <= right.first());

    quicksort(left);
    // slice everything after the first element(pivot)
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ unsorted | pivot | sorted ]
        quicksort(slice);
    }
}

#[test]
fn selection_works() {
    let mut slice = vec![2, 1, 7, 5, 3];
    QuickSort.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 5, 7]);
}
