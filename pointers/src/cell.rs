use crate::refcell::RefState;
use std::{cell::UnsafeCell, fmt::Debug};
pub struct Cell<T> {
    value: UnsafeCell<T>,
}
// impled by unsafe cell
// impl<T> !Sync for Cell<T> {}
impl Debug for Cell<RefState> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{:?}", unsafe { *self.value.get() });
        std::fmt::Result::Ok(())
    }
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // Safety: we know no-one else is concurrently mutating self.value because (!Sync)
        // Safety : we know we're not invalidating any refs, because we never give one out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // Safety: we know no-one else is modifying this value, since only this thread can mutate and i can execute  this function instead
        unsafe { *self.value.get() }
    }
}

// #[cfg(test)]
// mod test {
//     use std::{sync::Arc, thread};

//     use super::Cell;
//     #[test]
//     fn bad() {
//         let x = Arc::new(Cell::new(42));
//         let x1 = Arc::clone(&x);

//         let jh1 = thread::spawn(move || {
//             for _ in 0..10000 {
//                 let x = x1.get();
//                 x1.set(x + 1);
//             }
//         });

//         let x2 = Arc::clone(&x);

//         let jh2 = thread::spawn(move || {
//             for _ in 0..10000 {
//                 let x = x2.get();
//                 x2.set(x + 1);
//             }
//         });

//         jh1.join().unwrap();
//         jh2.join().unwrap();

//         assert_eq!(x.get(), 20000)
//     }
// }
