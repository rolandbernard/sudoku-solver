
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

    fn reduce_constraint(&mut self, i: usize, changed: &mut bool, changes: &mut [bool]) {
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
                    }
                }
            }
        }
    }

    fn reduce(&mut self, i: Option<usize>) {
        let mut changes;
        let mut changed;
        if let Some(i) = i {
            changes = vec![false; self.problem.constraints.len()];
            changed = false;
            for &c in &self.problem.constrained[i] {
                self.reduce_constraint(c, &mut changed, &mut changes);
            }
        } else {
            changes = vec![true; self.problem.constraints.len()];
            changed = true;
        }
        while changed {
            changed = false;
            for i in 0..self.problem.constraints.len() {
                if changes[i] {
                    changes[i] = false;
                    self.reduce_constraint(i, &mut changed, &mut changes);
                }
            }
        }
    }

    fn solve_next(&mut self, i: Option<usize>) -> bool {
        self.reduce(i);
        for v in &self.domains {
            if v.is_empty() {
                return false;
            }
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
                    copy.domains[i as usize] = DomainSet::singelton(j);
                    if !copy.solve() {
                        self.domains[i].remove(j);
                    }
                }
            }
        }
    }
}

