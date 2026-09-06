#![allow(dead_code)]

use std::{
    cmp::{Ordering, max},
    marker::PhantomData,
    ptr::NonNull,
    sync::{LazyLock, atomic::AtomicUsize},
};

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
        max(x, 1)
    } else {
        8
    }
});

struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    height: usize,
    next: Vec<Option<NonNull<Node<K, V>>>>,
}

impl<K, V> Node<K, V> {
    fn create(key: Option<K>, value: Option<V>, height: usize) -> Self {
        Self {
            key,
            value,
            height,
            next: vec![None; height],
        }
    }

    pub fn empty(height: usize) -> Self {
        Self::create(None, None, height)
    }

    pub fn new(key: K, value: V, height: usize) -> Self {
        Self::create(Some(key), Some(value), height)
    }
}

pub struct SkipList<K: Ord, V> {
    arena: Arena,
    head: NonNull<Node<K, V>>,
    max_height: AtomicUsize,
    phantom: PhantomData<(K, V)>,
}

impl<K: Ord, V> SkipList<K, V> {
    pub fn try_new() -> Option<Self> {
        let arena = Arena::new(*SIZE_LIMIT);
        let head = NonNull::from(arena.alloc(Node::empty(*MAX_HEIGHT))?);

        Some(Self {
            arena,
            head,
            max_height: AtomicUsize::new(1),
            phantom: PhantomData,
        })
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let height = self.max_height.load(std::sync::atomic::Ordering::Relaxed);

        let mut current = self.head;
        for level in (0..height).rev() {
            loop {
                let next = unsafe { current.as_ref() }.next[level];
                let Some(next) = next else { break };

                let ordering = unsafe { (*next.as_ptr()).key.as_ref().unwrap().cmp(key) };
                match ordering {
                    Ordering::Less => current = next,
                    Ordering::Equal => return unsafe { (*next.as_ptr()).value.as_ref() },
                    Ordering::Greater => break,
                }
            }
        }

        None
    }

    pub fn get_lower_bound(&self, target: &K) -> Option<(&K, &V)> {
        let mut current = self.head;
        let levels = unsafe { (&(*self.head.as_ptr()).next).len() };
        if levels == 0 {
            return None;
        }
        for level in (0..levels).rev() {
            loop {
                let next = unsafe { (&(*current.as_ptr()).next)[level] };
                let Some(next) = next else {
                    break;
                };
                let next_key = unsafe {
                    (&(*next.as_ptr()).key)
                        .as_ref()
                        .expect("non-head skip-list node has no key")
                };
                if next_key < target {
                    current = next;
                } else {
                    break;
                }
            }
        }
        let candidate = unsafe { (&(*current.as_ptr()).next)[0] }?;
        let node = unsafe { &*candidate.as_ptr() };
        let key = node
            .key
            .as_ref()
            .expect("non-head skip-list node has no key");
        let value = node
            .value
            .as_ref()
            .expect("non-head skip-list node has no value");
        Some((key, value))
    }

    pub fn put(&mut self, key: K, value: V) -> bool {
        let height_limit = unsafe { self.head.as_ref().height };
        let current_height = self.max_height.load(std::sync::atomic::Ordering::Relaxed);

        let mut update = vec![self.head; height_limit];
        let mut current = self.head;

        for level in (0..current_height).rev() {
            loop {
                let next = unsafe { (&(*current.as_ptr()).next)[level] };
                let Some(next) = next else { break };

                let ordering = unsafe {
                    (*next.as_ptr())
                        .key
                        .as_ref()
                        .expect("non-head skip-list node has no key")
                        .cmp(&key)
                };
                match ordering {
                    Ordering::Less => current = next,
                    Ordering::Equal => {
                        unsafe {
                            (*next.as_ptr()).value = Some(value);
                        }
                        return true;
                    }
                    Ordering::Greater => break,
                }
            }

            update[level] = current;
        }

        let node_height = random_height(height_limit);
        if node_height > current_height {
            for predecessor in &mut update[current_height..node_height] {
                *predecessor = self.head;
            }
        }

        let new_node = Node::new(key, value, node_height);
        let mut new_node = match self.arena.alloc(new_node) {
            Some(node) => NonNull::from(node),
            None => return false,
        };
        for level in 0..node_height {
            let mut predecessor = update[level];
            unsafe {
                let successor = predecessor.as_ref().next[level];
                new_node.as_mut().next[level] = successor;
                predecessor.as_mut().next[level] = Some(new_node);
            }
        }
        if node_height > current_height {
            self.max_height
                .store(node_height, std::sync::atomic::Ordering::Relaxed);
        }

        true
    }
}

impl<K: Ord, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        let mut current = Some(self.head);
        while let Some(node) = current {
            current = unsafe { node.as_ref() }.next[0];
            unsafe {
                std::ptr::drop_in_place(node.as_ptr());
            }
        }
    }
}

fn random_height(max_height: usize) -> usize {
    let mut height = 1;
    while height < max_height && fastrand::usize(0..4) == 0 {
        height += 1;
    }
    height
}

#[cfg(test)]
mod tests {
    use crate::skip_list::SkipList;

    #[test]
    fn test() {
        let mut list = SkipList::<i32, String>::try_new().unwrap();

        assert!(list.put(10, "ten".to_owned()));
        assert!(list.put(20, "twenty".to_owned()));
        assert!(list.put(15, "fifteen".to_owned()));

        assert_eq!(list.get(&10).map(String::as_str), Some("ten"));
        assert_eq!(list.get(&15).map(String::as_str), Some("fifteen"));
        assert_eq!(list.get(&99), None);

        // 更新已有 key，即使 Arena 已满也不需要新节点。
        assert!(list.put(10, "TEN".to_owned()));
        assert_eq!(list.get(&10).map(String::as_str), Some("TEN"));
    }
}
