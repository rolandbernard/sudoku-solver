use serde::{Deserialize, Serialize};
use std::ops::{BitAnd, BitOr, Not, Range};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct DomainSet {
    bitset: u64,
}

impl DomainSet {
    pub fn empty() -> Self {
        DomainSet { bitset: 0 }
    }

    pub fn singleton(e: u32) -> Self {
        let mut new = Self::empty();
        new.add(e);
        return new;
    }

    pub fn range(range: Range<u32>) -> Self {
        let mut new = Self::empty();
        for i in range {
            new.add(i);
        }
        return new;
    }

    pub fn contains(&self, e: u32) -> bool {
        self.bitset & (1 << e) != 0
    }

    pub fn add(&mut self, e: u32) {
        self.bitset |= 1 << e;
    }

    pub fn remove(&mut self, e: u32) {
        self.bitset &= !(1 << e);
    }

    pub fn without(&self, e: u32) -> Self {
        DomainSet {
            bitset: self.bitset & !(1 << e),
        }
    }

    pub fn without_all(&self, s: Self) -> Self {
        DomainSet {
            bitset: self.bitset & !s.bitset,
        }
    }

    pub fn add_all(&mut self, s: Self) {
        self.bitset |= s.bitset;
    }

    pub fn remove_all(&mut self, s: Self) {
        self.bitset &= !s.bitset;
    }

    pub fn retain_all(&mut self, s: Self) {
        self.bitset &= s.bitset;
    }

    pub fn is_singleton(&self) -> bool {
        self.bitset.is_power_of_two()
    }

    pub fn is_empty(&self) -> bool {
        self.bitset == 0
    }

    pub fn get_any(&self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            Some(self.bitset.trailing_zeros())
        }
    }

    pub fn len(&self) -> usize {
        self.bitset.count_ones() as usize
    }
}

impl BitAnd for DomainSet {
    type Output = DomainSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        DomainSet {
            bitset: self.bitset & rhs.bitset,
        }
    }
}

impl BitOr for DomainSet {
    type Output = DomainSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        DomainSet {
            bitset: self.bitset | rhs.bitset,
        }
    }
}

impl Not for DomainSet {
    type Output = DomainSet;

    fn not(self) -> Self::Output {
        DomainSet {
            bitset: !self.bitset,
        }
    }
}

impl Iterator for DomainSet {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        } else {
            let i = self.bitset.trailing_zeros();
            self.remove(i);
            return Some(i);
        }
    }
}
