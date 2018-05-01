/*! MVars for rust
*/

use std::sync::*;

pub struct MVar<T> {
    lock: Mutex<Option<T>>,
    takers: Condvar,
    putters: Condvar,
}

impl<T> MVar<T> {
    pub fn new(x: T) -> MVar<T> {
        MVar {
            lock: Mutex::new(Some(x)),
            takers: Condvar::new(),
            putters: Condvar::new(),
        }
    }

    pub fn new_empty() -> MVar<T> {
        MVar {
            lock: Mutex::new(None),
            takers: Condvar::new(),
            putters: Condvar::new(),
        }
    }

    pub fn take(&mut self) -> T {
        let mut guard = self.lock.lock().unwrap();
        loop {
            match guard.take() {
                Some(x) => {
                    self.putters.notify_one();
                    return x;
                }
                None => {}
            }
            guard = self.takers.wait(guard).unwrap();
        }
    }

    pub fn put(&mut self, x: T) {
        let mut guard = self.lock.lock().unwrap();
        loop {
            match *guard {
                None => {
                    *guard = Some(x);
                    ::std::mem::drop(guard);
                    self.takers.notify_one();
                    return;
                }
                Some(_) => {}
            }
            guard = self.putters.wait(guard).unwrap();
        }
    }

    pub fn modify<F: FnOnce(T) -> T>(&mut self, f: F) {
        let mut guard = self.lock.lock().unwrap();
        loop {
            match guard.take() {
                Some(x) => {
                    let x_2 = f(x);
                    *guard = Some(x_2);
                    ::std::mem::drop(guard);
                    self.takers.notify_one();
                    return;
                }
                None => {}
            }
            guard = self.takers.wait(guard).unwrap();
        }
    }
}
