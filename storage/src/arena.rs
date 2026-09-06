#![allow(dead_code)]

use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    mem::MaybeUninit,
    ptr::NonNull,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

#[derive(Clone)]
pub struct Arena {
    inner: Arc<ArenaInner>,
}

struct ArenaInner {
    data: UnsafeCell<Vec<u8>>,
    capacity: usize,
    offset: AtomicUsize,
}

unsafe impl Sync for ArenaInner {} // UnsafeCell is !Sync

impl Arena {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(ArenaInner {
                data: UnsafeCell::new(Vec::with_capacity(capacity)),
                capacity,
                offset: AtomicUsize::new(0),
            }),
        }
    }

    pub fn alloc<T: Sized>(&self, value: T) -> Option<&mut T> {
        self.alloc_uninit().map(|x| x.write(value))
    }

    pub fn alloc_uninit<T: Sized>(&self) -> Option<&mut MaybeUninit<T>> {
        let size = size_of::<T>();
        if size == 0 {
            let ptr = NonNull::<MaybeUninit<T>>::dangling().as_ptr();
            return Some(unsafe { &mut *ptr });
        }

        let base_ptr = unsafe { (*self.inner.data.get()).as_mut_ptr() };
        let mut current = self.inner.offset.load(Ordering::Relaxed);

        loop {
            if current > self.inner.capacity {
                break None;
            }

            let padding = unsafe { base_ptr.add(current) }.align_offset(align_of::<T>());
            if padding == usize::MAX {
                break None;
            }

            let aligned_offset = current.checked_add(padding)?;
            let new_offset = aligned_offset.checked_add(size)?;
            if new_offset > self.inner.capacity {
                break None;
            }

            match self.inner.offset.compare_exchange_weak(
                current,
                new_offset,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let ptr = unsafe { base_ptr.add(aligned_offset).cast::<MaybeUninit<T>>() };
                    break Some(unsafe { &mut *ptr });
                }
                Err(actual) => {
                    current = actual;
                    spin_loop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::arena::Arena;

    #[derive(Debug, PartialEq, Eq)]
    struct A(i32);

    #[test]
    fn test() {
        let arena = Arena::new(9);

        let x = arena.alloc::<A>(A(123));
        assert_eq!(123, x.unwrap().0);

        let y = arena.alloc::<A>(A(456));
        assert_eq!(456, y.unwrap().0);

        assert!(arena.alloc::<i32>(0).is_none());
    }

    #[test]
    fn test_uninit() {
        let arena = Arena::new(5);

        let x = arena.alloc_uninit::<A>();
        assert_eq!(A(123), *x.unwrap().write(A(123)));

        assert!(arena.alloc::<i32>(0).is_none())
    }
}
