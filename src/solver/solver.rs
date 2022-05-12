
use super::domain::DomainSet;

#[derive(Clone)]
pub struct Problem {
    pub variables: Vec<DomainSet>,
    pub constraints: Vec<Vec<usize>>,
}

impl Problem {
    pub fn empty() -> Problem {
        Problem { variables: Vec::new(), constraints: Vec::new() }
    }

    pub fn add_variable(&mut self, domain: DomainSet) -> usize {
        self.variables.push(domain);
        return self.variables.len() - 1;
    }

    pub fn add_constraint(&mut self, constraint: Vec<usize>) {
        self.constraints.push(constraint);
    }

    // Local domain consistency
    pub fn reduce_domains(&mut self) {
        let mut repeat = true;
        while repeat {
            repeat = false;
            for c in &self.constraints {
                for &v in c {
                    let dom = self.variables[v];
                    if dom.is_singelton() {
                        for &w in c {
                            if v != w && !(dom & self.variables[w]).is_empty() {
                                repeat = true;
                                self.variables[w].remove_all(dom);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn reduced_domains(&self) -> Problem {
        let mut copy = self.clone();
        copy.reduce_domains();
        return copy;
    }

    // Global satisfiability
    pub fn solve(&mut self) -> bool {
        self.reduce_domains();
        for v in &self.variables {
            if v.is_empty() {
                return false;
            }
        }
        for (i, v) in self.variables.iter().enumerate() {
            if !v.is_singelton() {
                for j in v.clone() {
                    let mut copy = self.clone();
                    copy.variables[i as usize] = DomainSet::singelton(j);
                    if copy.solve() {
                        self.variables = copy.variables;
                        return true;
                    }
                }
            }
        }
        return true;
    }

    pub fn find_model(&self) -> Option<Problem> {
        let mut copy = self.clone();
        if copy.solve() {
            return Some(copy);
        } else {
            return None;
        }
    }

    // Global domain consistency
    pub fn minimize_domains(&mut self) {
        self.reduce_domains();
        for (i, v) in self.variables.clone().iter().enumerate() {
            if !v.is_empty() && !v.is_singelton() {
                for j in v.clone() {
                    let mut copy = self.clone();
                    copy.variables[i as usize] = DomainSet::singelton(j);
                    if !copy.solve() {
                        self.variables[i].remove(j);
                    }
                }
            }
        }
    }

    pub fn minimized_domains(&mut self) -> Problem {
        let mut copy = self.clone();
        copy.minimize_domains();
        return copy;
    }
}

