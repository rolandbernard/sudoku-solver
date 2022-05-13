
use super::{solver::Problem, domain::DomainSet};

pub type Sudoku = [[Option<u32>; 9]; 9];

pub fn empty_sudoku() -> Sudoku {
    [[Option::<u32>::None; 9]; 9]
}

pub fn create_problem(sudoku: &Sudoku) -> Problem {
    let mut prob = Problem::empty();
    for row in sudoku {
        for cel in row {
            if let Some(v) = cel {
                prob.add_variable(DomainSet::singelton(*v - 1));
            } else {
                prob.add_variable(DomainSet::range(0..9));
            }
        }
    }
    for i in 0..9 {
        let mut row = Vec::new();
        let mut col = Vec::new();
        let mut cell = Vec::new();
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

pub fn update_variables(sudoku: &Sudoku, problem: &mut Problem) {
    for (i, row) in sudoku.iter().enumerate() {
        for (j, cel) in row.iter().enumerate() {
            if let Some(v) = cel {
                problem.variables[9*i + j] = DomainSet::singelton(*v);
            } else {
                problem.variables[9*i + j] = DomainSet::range(0..9);
            }
        }
    }
}

pub fn read_solution(sudoku: &mut Sudoku, problem: &Problem) {
    for i in 0..9 {
        for j in 0..9 {
            sudoku[i][j] = problem.variables[9*i + j].get_any().and_then(|x| Some(x + 1));
        }
    }
}

