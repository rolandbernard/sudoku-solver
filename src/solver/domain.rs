
use std::ops::{BitAnd, BitOr};

pub struct DomainSet {
    bitset: u32,
}

impl DomainSet {
    pub fn empty() -> DomainSet {
        DomainSet { bitset: 0 }
    }

    pub fn range(a: u32, b: u32) -> DomainSet {
        let mut new = Self::empty();
        for i in a..b {
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

    pub fn add_all(&mut self, s: DomainSet) {
        self.bitset |= s.bitset;
    }

    pub fn remove_all(&mut self, s: DomainSet) {
        self.bitset &= !s.bitset;
    }

    pub fn retain_all(&mut self, s: DomainSet) {
        self.bitset &= s.bitset;
    }
}

impl BitAnd for DomainSet {
    type Output = DomainSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        DomainSet { bitset: self.bitset & rhs.bitset }
    }
}

impl BitOr for DomainSet {
    type Output = DomainSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        DomainSet { bitset: self.bitset | rhs.bitset }
    }
}

