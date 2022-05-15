
use crate::solver::domain::DomainSet;

pub struct Problem {
    domains: Vec<DomainSet>,
    constrained: Vec<Vec<usize>>,
    constraints: Vec<Vec<usize>>,
}

impl Problem {
    pub fn empty() -> Problem {
        Problem { domains: Vec::new(), constrained: Vec::new(), constraints: Vec::new(), }
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
            return state.domains.into_iter()
                .map(|v| v.get_any())
                .collect();
        } else {
            return None;
        }
    }

    pub fn reduce_domains(&self) -> Vec<DomainSet> {
        let mut state = ProblemState::from_problem(self);
        state.reduce(None);
        return state.domains;
    }

    pub fn minimized_domains(&self) -> Vec<DomainSet> {
        let mut state = ProblemState::from_problem(self);
        state.minimize();
        return state.domains;
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

    fn reduce_constraint(&mut self, i: usize, changed: &mut bool, changes: &mut [bool]) -> bool {
        let constr_len = self.problem.constraints[i].len();
        let mut all = DomainSet::empty();
        for &v in &self.problem.constraints[i] {
            all.add_all(self.domains[v]);
        }
        if all.len() < constr_len {
            return false;
        }
        for &v in &self.problem.constraints[i] {
            let dom = self.domains[v];
            if dom.is_singelton() {
                for &w in &self.problem.constraints[i] {
                    if v != w && !(dom & self.domains[w]).is_empty() {
                        for &c in &self.problem.constrained[w] {
                            changes[c] = true;
                        }
                        *changed = true;
                        self.domains[w].remove_all(dom);
                        if self.domains[w].is_empty() {
                            return false;
                        }
                    }
                }
            }
        }
        for &v in &self.problem.constraints[i] {
            let mut without = DomainSet::empty();
            for &w in &self.problem.constraints[i] {
                if v != w {
                    without.add_all(self.domains[w]);
                }
            }
            let old = self.domains[v].clone();
            for i in self.domains[v].clone() {
                if without.without(i).len() < constr_len - 1 {
                    self.domains[v].remove(i);
                }
            }
            if self.domains[v].is_empty() {
                return false;
            } else if self.domains[v] != old {
                for &c in &self.problem.constrained[v] {
                    changes[c] = true;
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
            changes = vec![false; self.problem.constraints.len()];
            for &c in &self.problem.constrained[i] {
                changes[c] = true;
            }
        } else {
            changes = vec![true; self.problem.constraints.len()];
        }
        while changed {
            changed = false;
            for i in 0..self.problem.constraints.len() {
                if changes[i] {
                    changes[i] = false;
                    if !self.reduce_constraint(i, &mut changed, &mut changes) {
                        self.domains = vec![DomainSet::empty(); self.domains.len()];
                        return false;
                    }
                }
            }
        }
        return true
    }

    fn solve_next(&mut self, i: Option<usize>) -> bool {
        if !self.reduce(i) {
            return false;
        }
        for (i, v) in self.domains.iter().enumerate() {
            if !v.is_singelton() {
                for j in v.clone() {
                    let mut copy = self.clone();
                    copy.domains[i] = DomainSet::singelton(j);
                    if copy.solve_next(Some(i)) {
                        self.domains = copy.domains;
                        return true;
                    }
                }
                return false;
            }
        }
        return true;
    }
    
    fn solve(&mut self) -> bool {
        self.solve_next(None)
    }

    fn minimize(&mut self) {
        self.reduce(None);
        for (i, v) in self.domains.clone().iter().enumerate() {
            if !v.is_empty() && !v.is_singelton() {
                for j in v.clone() {
                    let mut copy = self.clone();
                    copy.domains[i] = DomainSet::singelton(j);
                    if !copy.solve_next(Some(i)) {
                        self.domains[i].remove(j);
                    }
                }
            }
        }
    }
}

