#![allow(dead_code)]

use std::sync::{LazyLock, atomic::AtomicUsize};

use crate::arena::Arena;

static SIZE_LIMIT: LazyLock<usize> = LazyLock::new(|| {
    if let Ok(x) = std::env::var("SKIP_LIST_SIZE_LIMIT")
        && let Ok(x) = x.parse()
    {
        x
    } else {
        1024 * 1024
    }
});

static MAX_HEIGHT: LazyLock<usize> = LazyLock::new(|| {
    if let Ok(x) = std::env::var("SKIP_LIST_MAX_HEIGHT")
        && let Ok(x) = x.parse()
    {
        x
    } else {
        8
    }
});

struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    height: usize,
    next: Vec<*const Node<K, V>>,
}

impl<K, V> Node<K, V> {
    fn create(key: Option<K>, value: Option<V>, height: usize) -> Self {
        Self {
            key,
            value,
            height,
            next: vec![std::ptr::null_mut(); height],
        }
    }

    pub fn empty(height: usize) -> Self {
        Self::create(None, None, height)
    }

    pub fn new(key: K, value: V, height: usize) -> Self {
        Self::create(Some(key), Some(value), height)
    }
}

pub struct SkipList<'a, K, V>
where
    K: Ord,
{
    arena: Arena,
    head: &'a mut Node<K, V>,
    max_height: AtomicUsize,
}

impl<'a, K, V> SkipList<'a, K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        // let arena = Arena::new(*SIZE_LIMIT);

        // let head = arena.alloc(Node::empty(*MAX_HEIGHT)).unwrap(); // FIXME: unwrap

        // Self {
        //     arena,
        //     head,
        //     max_height: AtomicUsize::new(1),
        // }
        todo!()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        todo!()
    }

    pub fn put(&mut self, key: K, value: V) -> bool {
        todo!()
    }
}

fn random_height() -> usize {
    let mut height: usize = 1;

    while height < *MAX_HEIGHT && fastrand::usize(0..4) == 0 {
        height += 1
    }

    height
}
