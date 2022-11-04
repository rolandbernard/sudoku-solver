#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::solver::domain::DomainSet;

pub struct Problem {
    domains: Vec<DomainSet>,
    constrained: Vec<Vec<usize>>,
    constraints: Vec<Vec<usize>>,
}

impl Problem {
    pub fn empty() -> Problem {
        Problem {
            domains: Vec::new(),
            constrained: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn with_capacity(variables: usize, constraints: usize) -> Problem {
        Problem {
            domains: Vec::with_capacity(variables),
            constrained: Vec::with_capacity(variables),
            constraints: Vec::with_capacity(constraints),
        }
    }

    pub fn add_variable(&mut self, domain: DomainSet) -> usize {
        self.domains.push(domain);
        self.constrained.push(Vec::new());
        return self.domains.len() - 1;
    }

    pub fn add_constraint(&mut self, constraint: Vec<usize>) {
        for &v in &constraint {
            self.constrained[v].push(self.constraints.len());
        }
        self.constraints.push(constraint);
    }

    pub fn find_model(&self) -> Option<Vec<u32>> {
        let mut state = ProblemState::from_problem(self);
        if state.solve() {
            return state.domains.into_iter().map(|v| v.get_any()).collect();
        } else {
            return None;
        }
    }

    pub fn reduced_domains(&self) -> Vec<DomainSet> {
        let mut state = ProblemState::from_problem(self);
        state.reduce(None);
        return state.domains;
    }

    pub fn minimized_domains(&self) -> Vec<DomainSet> {
        let mut state = ProblemState::from_problem(self);
        state.minimize();
        return state.domains;
    }

    pub fn minimize_domains_for(
        &self,
        mut unsure: Vec<DomainSet>,
        timeout: u64,
    ) -> (Vec<DomainSet>, Vec<DomainSet>) {
        let mut state = ProblemState::from_problem(self);
        state.minimize_for(&mut unsure, timeout);
        return (state.domains, unsure);
    }
}

struct Random {
    state: u128,
}

impl Random {
    fn new() -> Self {
        Random {
            state: Self::get_time() as u128 | (Self::get_time() as u128) << 64,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn get_time() -> u64 {
        js_sys::Date::now() as u64
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_time() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn get_random(&mut self) -> usize {
        let mut hash = DefaultHasher::new();
        self.state.hash(&mut hash);
        let h1 = hash.finish();
        self.state.hash(&mut hash);
        let h2 = hash.finish();
        self.state.hash(&mut hash);
        let h3 = hash.finish();
        self.state = h1 as u128 | (h2 as u128) << 64;
        return h3 as usize;
    }
}

#[derive(Clone)]
struct ProblemState<'a> {
    problem: &'a Problem,
    domains: Vec<DomainSet>,
}

impl<'a> ProblemState<'a> {
    fn from_problem(problem: &'a Problem) -> Self {
        ProblemState {
            problem,
            domains: problem.domains.clone(),
        }
    }

    fn is_constr_satisfiable(
        &self,
        constr: &[usize],
        mut remove: DomainSet,
        mut taken: DomainSet,
    ) -> bool {
        let mut left = constr.len() - taken.len();
        let mut change = true;
        while change && left > 0 {
            change = false;
            for (i, &w) in constr.iter().enumerate() {
                if !taken.contains(i as u32) {
                    let rem = self.domains[w].without_all(remove);
                    if rem.is_singleton() {
                        remove.add_all(rem);
                        taken.add(i as u32);
                        left -= 1;
                        change = true;
                    } else if rem.is_empty() {
                        return false;
                    }
                }
            }
        }
        let mut without = DomainSet::empty();
        left = 0;
        for (i, &w) in constr.iter().enumerate() {
            if !taken.contains(i as u32) {
                left += 1;
                without.add_all(self.domains[w]);
                if without.without_all(remove).len() < left {
                    return false;
                }
            }
        }
        return true;
    }

    fn reduce_constraint(&mut self, constr: &[usize], changes: &mut DomainSet) -> bool {
        for (i, &v) in constr.iter().enumerate() {
            let old = self.domains[v];
            for j in self.domains[v] {
                if !self.is_constr_satisfiable(
                    constr,
                    DomainSet::singleton(j),
                    DomainSet::singleton(i as u32),
                ) {
                    self.domains[v].remove(j);
                    if self.domains[v].is_empty() {
                        return false;
                    }
                }
            }
            if self.domains[v] != old {
                for &c in &self.problem.constrained[v] {
                    changes.add(c as u32);
                }
            }
        }
        return true;
    }

    fn reduce(&mut self, i: Option<usize>) -> bool {
        let mut changes;
        if let Some(i) = i {
            changes = DomainSet::empty();
            for &c in &self.problem.constrained[i] {
                changes.add(c as u32);
            }
        } else {
            changes = DomainSet::range(0..self.problem.constraints.len() as u32);
        }
        while !changes.is_empty() {
            for i in changes.clone() {
                changes.remove(i as u32);
                if !self.reduce_constraint(&self.problem.constraints[i as usize], &mut changes) {
                    self.domains = vec![DomainSet::empty(); self.domains.len()];
                    return false;
                }
            }
        }
        return true;
    }

    fn solve(&mut self) -> bool {
        return self.reduce(None)
            && self
                .solve_with(
                    &mut Random::new(),
                    &self.problem.domains,
                    &mut None,
                    &mut usize::MAX.clone(),
                )
                .unwrap_or(false);
    }

    fn solve_selecting(
        &mut self,
        random: &mut Random,
        prefer: &[DomainSet],
        backtracked_on: &mut Option<usize>,
        backtracked_count: &mut usize,
        i: usize,
    ) -> Option<bool> {
        if *backtracked_count == 0 {
            return None;
        } else {
            let v = self.domains[i];
            for j in (v & prefer[i]).chain(v.without_all(prefer[i])) {
                let mut copy = self.clone();
                copy.domains[i] = DomainSet::singleton(j);
                if copy.reduce(Some(i)) {
                    if let Some(result) =
                        copy.solve_with(random, prefer, backtracked_on, backtracked_count)
                    {
                        if result {
                            self.domains = copy.domains;
                            return Some(true);
                        }
                    } else {
                        return None;
                    }
                }
            }
            if *backtracked_on == None {
                *backtracked_on = Some(i);
            }
            if *backtracked_count > 0 {
                *backtracked_count -= 1;
            }
            return Some(false);
        }
    }

    fn solve_with(
        &mut self,
        random: &mut Random,
        prefer: &[DomainSet],
        backtracked_on: &mut Option<usize>,
        backtracked_count: &mut usize,
    ) -> Option<bool> {
        if let Some(i) = *backtracked_on {
            return self.solve_selecting(random, prefer, &mut None, backtracked_count, i);
        } else {
            let off = random.get_random() % self.domains.len();
            for i in (off..self.domains.len()).chain(0..off) {
                let v = self.domains[i];
                if !v.is_singleton() {
                    return self.solve_selecting(
                        random,
                        prefer,
                        backtracked_on,
                        backtracked_count,
                        i,
                    );
                }
            }
            return Some(true);
        }
    }

    fn minimize_for(&mut self, unsure: &mut [DomainSet], timeout: u64) {
        let start = Random::get_time();
        let mut random = Random::new();
        let off = random.get_random() % self.domains.len();
        for i in (off..self.domains.len()).chain(0..off) {
            unsure[i].retain_all(self.domains[i]);
            for j in unsure[i] {
                if self.domains[i].contains(j) {
                    let mut copy = self.clone();
                    copy.domains[i] = DomainSet::singleton(j);
                    let result = Some(self.reduce(Some(i))).and_then(|x| {
                        if x {
                            copy.solve_with(&mut random, &unsure, &mut None, &mut 128)
                        } else {
                            Some(false)
                        }
                    });
                    if let Some(solved) = result {
                        if solved {
                            for (i, &d) in copy.domains.iter().enumerate() {
                                unsure[i].remove_all(d);
                            }
                        } else {
                            self.domains[i].remove(j);
                            self.reduce(Some(i));
                        }
                    }
                }
                let elapsed_time = Random::get_time() - start;
                if elapsed_time > timeout {
                    return;
                }
            }
        }
    }

    fn minimize(&mut self) {
        self.reduce(None);
        let mut unsure = self.domains.clone();
        self.minimize_for(&mut unsure, u64::MAX);
    }
}
