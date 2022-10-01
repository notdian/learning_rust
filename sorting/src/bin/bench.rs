use rand::{seq::SliceRandom, Rng};
use sorting::bubblesort::BubbleSort;
use sorting::insertionsort::InsertionSort;
use sorting::quicksort::QuickSort;
use sorting::selectionsort::SelectionSort;
use sorting::Sorter;
use std::{cell::Cell, rc::Rc};

#[derive(Clone, Eq)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    for &n in &[0, 1, 10, 100, 100, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);
            let took = bench(BubbleSort, &values, &counter);
            println!("{},{},{}", "BubbleSort", n, took);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("{},{},{}", "InsertionSort Dumb", n, took);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("{},{},{}", "InsertionSort Smart", n, took);
            let took = bench(
                SelectionSort {
                    use_iterator: false,
                },
                &values,
                &counter,
            );
            println!("{},{},{}", "SelectionSort", n, took);
            let took = bench(SelectionSort { use_iterator: true }, &values, &counter);
            println!("{},{},{}", "SelectionSort iter", n, took);
            let took = bench(QuickSort, &values, &counter);
            println!("{},{},{}", "QuickSort", n, took);
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    counter.get()
}
