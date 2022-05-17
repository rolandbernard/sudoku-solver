
use crate::solver::{solver::Problem, domain::DomainSet};

pub type Sudoku = [[Option<u32>; 9]; 9];
pub type SudokuDomains = [[DomainSet; 9]; 9];

pub fn empty_sudoku() -> Sudoku {
    [[Option::<u32>::None; 9]; 9]
}

pub fn empty_domains() -> SudokuDomains {
    [[DomainSet::empty(); 9]; 9]
}

pub fn default_domains() -> SudokuDomains {
    [[DomainSet::range(0..9); 9]; 9]
}

pub fn sudoku_domains(sudoku: &Sudoku) -> SudokuDomains {
    let mut res = empty_domains();
    for i in 0..9 {
        for j in 0..9 {
            if let Some(v) = sudoku[i][j] {
                res[i][j] = DomainSet::singelton(v);
            } else {
                res[i][j] = DomainSet::range(0..9);
            }
        }
    }
    return res;
}

pub fn create_problem(sudoku: &SudokuDomains) -> Problem {
    let mut prob = Problem::with_capacity(9*9, 3*9);
    for row in sudoku {
        for cel in row {
            prob.add_variable(*cel);
        }
    }
    for i in 0..9 {
        let mut row = Vec::with_capacity(9);
        let mut col = Vec::with_capacity(9);
        let mut cell = Vec::with_capacity(9);
        for j in 0..9 {
            row.push(9 * i + j);
            col.push(9 * j + i);
            let ii = 3 * (i / 3) + j / 3;
            let jj = 3 * (i % 3) + j % 3;
            cell.push(9 * ii + jj)
        }
        prob.add_constraint(row);
        prob.add_constraint(col);
        prob.add_constraint(cell);
    }
    return prob;
}

pub fn resize_variables(variables: Vec<u32>) -> Sudoku {
    let mut res = empty_sudoku();
    for (i, v) in variables.iter().enumerate() {
        res[i / 9][i % 9] = Some(*v + 1);
    }
    return res;
}

pub fn resize_domains(domains: Vec<DomainSet>) -> SudokuDomains {
    let mut res = empty_domains();
    for (i, v) in domains.iter().enumerate() {
        res[i / 9][i % 9] = *v;
    }
    return res;
}

