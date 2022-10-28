use crate::solver::{domain::DomainSet, solver::Problem};

pub type Sudoku<const N: usize> = [[Option<u32>; N]; N];
pub type SudokuDomains<const N: usize> = [[DomainSet; N]; N];

pub fn empty_sudoku<const N: usize>() -> Sudoku<N> {
    [[Option::<u32>::None; N]; N]
}

pub fn empty_domains<const N: usize>() -> SudokuDomains<N> {
    [[DomainSet::empty(); N]; N]
}

pub fn default_domains<const N: usize>() -> SudokuDomains<N> {
    [[DomainSet::range(0..N as u32); N]; N]
}

pub fn sudoku_domains<const N: usize>(sudoku: &Sudoku<N>) -> SudokuDomains<N> {
    let mut res = empty_domains();
    for i in 0..N {
        for j in 0..N {
            if let Some(v) = sudoku[i][j] {
                res[i][j] = DomainSet::singleton(v - 1);
            } else {
                res[i][j] = DomainSet::range(0..N as u32);
            }
        }
    }
    return res;
}

pub fn create_problem<const N: usize>(sudoku: &SudokuDomains<N>) -> Problem {
    let mut prob = Problem::with_capacity(N * N, 3 * N);
    for row in sudoku {
        for cel in row {
            prob.add_variable(*cel);
        }
    }
    for i in 0..N {
        let mut row = Vec::with_capacity(N);
        let mut col = Vec::with_capacity(N);
        let mut cell = Vec::with_capacity(N);
        for j in 0..N {
            row.push(N * i + j);
            col.push(N * j + i);
            let sr = (N as f64).sqrt() as usize;
            let ii = sr * (i / sr) + j / sr;
            let jj = sr * (i % sr) + j % sr;
            cell.push(N * ii + jj)
        }
        prob.add_constraint(row);
        prob.add_constraint(col);
        prob.add_constraint(cell);
    }
    return prob;
}

pub fn resize_variables<const N: usize>(variables: Vec<u32>) -> Sudoku<N> {
    let mut res = empty_sudoku();
    for (i, v) in variables.iter().enumerate() {
        res[i / N][i % N] = Some(*v + 1);
    }
    return res;
}

pub fn resize_domains<const N: usize>(domains: Vec<DomainSet>) -> SudokuDomains<N> {
    let mut res = empty_domains();
    for (i, v) in domains.iter().enumerate() {
        res[i / N][i % N] = *v;
    }
    return res;
}
