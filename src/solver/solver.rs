#[cfg(not(target_arch="wasm32"))]
use std::time::SystemTime;

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
        if state.solve(None) {
            return state.domains.into_iter().map(|v| v.get_any()).collect();
        } else {
            return None;
        }
    }

    pub fn find_model_with(&self, prefer: &[DomainSet]) -> Option<Vec<u32>> {
        let mut state = ProblemState::from_problem(self);
        if state.solve_with(None, prefer) {
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

#[derive(Clone)]
struct ProblemState<'a> {
    problem: &'a Problem,
    domains: Vec<DomainSet>,
}

impl<'a> ProblemState<'a> {
    fn from_problem(problem: &'a Problem) -> Self {
        ProblemState {
            problem: problem,
            domains: problem.domains.clone(),
        }
    }

    fn reduce_singletons(
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
        for (i, &w) in constr.iter().enumerate() {
            if !taken.contains(i as u32) {
                without.add_all(self.domains[w]);
            }
        }
        if without.without_all(remove).len() < left {
            return false;
        } else {
            return true;
        }
    }

    fn reduce_constraint(
        &mut self,
        constr: &[usize],
        changed: &mut bool,
        changes: &mut DomainSet,
    ) -> bool {
        for (i, &v) in constr.iter().enumerate() {
            let old = self.domains[v];
            for j in self.domains[v] {
                if !self.reduce_singletons(
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
                *changed = true;
            }
        }
        return true;
    }

    fn reduce(&mut self, i: Option<usize>) -> bool {
        let mut changes;
        let mut changed = true;
        if let Some(i) = i {
            changes = DomainSet::empty();
            for &c in &self.problem.constrained[i] {
                changes.add(c as u32);
            }
        } else {
            changes = DomainSet::range(0..self.problem.constraints.len() as u32);
        }
        while changed {
            changed = false;
            for (i, constr) in self.problem.constraints.iter().enumerate() {
                if changes.contains(i as u32) {
                    changes.remove(i as u32);
                    if !self.reduce_constraint(constr, &mut changed, &mut changes) {
                        self.domains = vec![DomainSet::empty(); self.domains.len()];
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn solve(&mut self, i: Option<usize>) -> bool {
        self.solve_with(i, &self.problem.domains)
    }

    fn solve_with(&mut self, i: Option<usize>, prefer: &[DomainSet]) -> bool {
        if !self.reduce(i) {
            return false;
        }
        for (i, v) in self.domains.iter().enumerate() {
            if !v.is_singleton() {
                for j in (*v & prefer[i]).chain(v.without_all(prefer[i])) {
                    let mut copy = self.clone();
                    copy.domains[i] = DomainSet::singleton(j);
                    if copy.solve_with(Some(i), prefer) {
                        self.domains = copy.domains;
                        return true;
                    }
                }
                return false;
            }
        }
        return true;
    }

    #[cfg(target_arch="wasm32")]
    fn get_time() -> u64 {
        js_sys::Date::now() as u64
    }
    
    #[cfg(not(target_arch="wasm32"))]
    fn get_time() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64
    }

    fn minimize_for(&mut self, unsure: &mut [DomainSet], timeout: u64) {
        let start = Self::get_time();
        self.reduce(None);
        for i in 0..self.domains.len() {
            unsure[i] = unsure[i] & self.domains[i];
        }
        for i in 0..self.domains.len() {
            for j in unsure[i] {
                let mut copy = self.clone();
                copy.domains[i] = DomainSet::singleton(j);
                if copy.solve_with(Some(i), &unsure) {
                    for (i, d) in copy.domains.iter().enumerate() {
                        unsure[i].remove_all(*d);
                    }
                } else {
                    self.domains[i].remove(j);
                }
                let elapsed_time = Self::get_time() - start;
                if elapsed_time > timeout {
                    return;
                }
            }
        }
    }

    fn minimize(&mut self) {
        let mut unsure = self.domains.clone();
        self.minimize_for(&mut unsure, u64::MAX);
    }
}
