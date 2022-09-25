use crate::cell::Cell;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

#[derive(Copy, Clone, Debug)]
pub enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}
#[derive(Debug)]
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // Safety: no ex refs have been given out, since state would be EXCLUSIVE
                Some(Ref { refcell: self })
            }
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }
    pub fn borrow_mut(&mut self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // Safety: no other refs have been given out, since state would be Shared or ex
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //Safertyt
        // a Ref i sonly create if no exclusive refrences are given out
        // State is shared
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => unreachable!(),
            RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
        }
    }
}
#[derive(Debug)]
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //Safertyt
        // a RefMut is only create if no other exclusive refrences are given out
        // State is exclusivem
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        //Safertyt
        // a Ref i sonly create if no exclusive refrences are given out
        // State is shared
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared => unreachable!(),
            RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cell::Cell;

    use super::RefCell;

    #[test]
    fn test1() {
        let mut rc = Cell::new(String::from("hello"));

        rc.set(String::from("'"));

        println!("{:?}", rc)
    }
}
