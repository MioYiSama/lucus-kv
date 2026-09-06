#![allow(dead_code)]

use std::sync::atomic::AtomicU64;

use bytes::Bytes;

use crate::skip_list::SkipList;

#[derive(PartialEq, Eq)]
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.user_key
            .cmp(&other.user_key)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
}

pub struct MemTable<'a> {
    counter: AtomicU64,
    skip_list: SkipList<'a, InternalKey, Bytes>,
}

impl<'a> MemTable<'a> {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            skip_list: SkipList::new(),
        }
    }

    pub fn put(&mut self, key: &str, value: impl Into<Bytes>) -> bool {
        let key = InternalKey::new_put(key.to_owned(), self.seq());
        let value = value.into();

        self.skip_list.put(key, value)
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        todo!();
    }

    pub fn delete(&mut self, key: &str) {
        InternalKey::new_delete(key.to_owned(), self.seq());
        todo!()
    }

    #[inline(always)]
    fn seq(&self) -> u64 {
        self.counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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
