
use std::ops::{BitAnd, BitOr, Not, Range};

#[derive(Clone, Copy)]
pub struct DomainSet {
    bitset: u32,
}

impl DomainSet {
    pub fn empty() -> DomainSet {
        DomainSet { bitset: 0 }
    }

    pub fn singelton(e: u32) -> DomainSet {
        let mut new = Self::empty();
        new.add(e);
        return new;
    }

    pub fn range(range: Range<u32>) -> DomainSet {
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

    pub fn add_all(&mut self, s: DomainSet) {
        self.bitset |= s.bitset;
    }

    pub fn remove_all(&mut self, s: DomainSet) {
        self.bitset &= !s.bitset;
    }

    pub fn retain_all(&mut self, s: DomainSet) {
        self.bitset &= s.bitset;
    }

    pub fn is_singelton(&self) -> bool {
        self.bitset & (!self.bitset + 1) == self.bitset
    }

    pub fn is_empty(&self) -> bool {
        self.bitset == 0
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

impl Not for DomainSet {
    type Output = DomainSet;

    fn not(self) -> Self::Output {
        DomainSet { bitset: !self.bitset }
    }
}

impl Iterator for DomainSet {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None
        } else {
            let mut last = !self.bitset + 1;
            self.bitset &= !last;
            let mut i = 0;
            while last > 1 {
                last /= 2;
                i += 1;
            }
            return Some(i);
        }
    }
}

