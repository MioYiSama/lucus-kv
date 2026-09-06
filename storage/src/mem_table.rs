#![allow(dead_code)]

use std::{cmp::Ordering, sync::atomic::AtomicU64};

use bytes::Bytes;

use crate::skip_list::SkipList;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ValueType {
    Delete = 0,
    Put = 1,
}

#[derive(PartialEq, Eq)]
struct InternalKey {
    user_key: String,
    seq: u64,
    value_type: ValueType,
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.user_key
            .cmp(&other.user_key)
            .then_with(|| other.seq.cmp(&self.seq))
            .then_with(|| other.value_type.cmp(&self.value_type))
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl InternalKey {
    fn new_put(user_key: String, seq: u64) -> Self {
        Self {
            user_key,
            seq,
            value_type: ValueType::Put,
        }
    }

    fn new_delete(user_key: String, seq: u64) -> Self {
        Self {
            user_key,
            seq,
            value_type: ValueType::Delete,
        }
    }

    fn new_lookup(user_key: String, seq: u64) -> Self {
        Self {
            user_key,
            seq,
            value_type: ValueType::Put,
        }
    }
}

pub struct MemTable {
    counter: AtomicU64,
    skip_list: SkipList<InternalKey, Bytes>,
}

impl<'a> MemTable {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            skip_list: SkipList::try_new().unwrap(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Bytes> {
        self.get_at(key, self.latest_seq())
    }

    pub fn get_at(&self, key: &str, seq: u64) -> Option<&Bytes> {
        let lookup_key = InternalKey::new_lookup(key.to_owned(), seq);
        let (internal_key, value) = self.skip_list.get_lower_bound(&lookup_key)?;

        if internal_key.user_key != key {
            return None;
        }

        match internal_key.value_type {
            ValueType::Put => Some(value),
            ValueType::Delete => None,
        }
    }

    pub fn put(&mut self, key: &str, value: impl Into<Bytes>) -> bool {
        let key = InternalKey::new_put(key.to_owned(), self.allocate_seq());
        self.skip_list.put(key, value.into())
    }

    pub fn delete(&mut self, key: &str) {
        let key = InternalKey::new_delete(key.to_owned(), self.allocate_seq());
        self.skip_list.put(key, Bytes::new());
    }

    #[inline(always)]
    pub fn latest_seq(&self) -> u64 {
        self.counter.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[inline(always)]
    fn allocate_seq(&self) -> u64 {
        let previous = self
            .counter
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |current| current.checked_add(1),
            )
            .expect("MemTable sequence number overflow");

        previous + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::mem_table::MemTable;

    #[test]
    fn test() {
        let mut mem_table = MemTable::new();
        mem_table.put("name", "admin");
        mem_table.put("age", "18");
        println!("{:?}", mem_table.get("name"));
        println!("{:?}", mem_table.get("name1"));
        mem_table.delete("name");
        println!("{:?}", mem_table.get("name"));
    }
}
