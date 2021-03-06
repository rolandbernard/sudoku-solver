
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::solver::domain::DomainSet;
use crate::solver::sudoku::{empty_sudoku, default_domains, Sudoku, SudokuDomains, sudoku_domains};
use crate::workers::{SolvingWorker, ReducingWorker, MinimizingWorker};
use crate::components::sudoku_input::SudokuInput;

pub enum SolverMessage {
    Change(Sudoku),
    Solve,
    Clear,
    Solved(Option<Sudoku>, usize),
    Reduced(SudokuDomains, usize),
    Minimized(SudokuDomains, usize),
    Undo,
    Redo,
}

#[derive(Clone)]
pub struct SudokuHistoryItem {
    sudoku: Sudoku,
    domains: SudokuDomains,
    change: usize,
    prog: i32,
    solved: Option<bool>,
}

impl SudokuHistoryItem {
    fn default() -> Self {
        SudokuHistoryItem {
            sudoku: empty_sudoku(),
            domains: default_domains(),
            change: 0,
            prog: 2,
            solved: None,
        }
    }
}

pub struct SudokuSolver {
    history: Vec<SudokuHistoryItem>,
    hist_pos: usize,
    change: usize,
    solving: Option<usize>,
    reducing: Option<usize>,
    minimizing: Option<usize>,
    minimize_bridge: Box<dyn Bridge<MinimizingWorker>>,
    reduce_bridge: Box<dyn Bridge<ReducingWorker>>,
    solver_bridge: Box<dyn Bridge<SolvingWorker>>,
}

fn count_domain_values(domains: &SudokuDomains) -> usize {
    let mut res = 0;
    for row in domains {
        for cell in row {
            res += cell.len();
        }
    }
    return res;
}

fn is_sudoku_subset(new: &Sudoku, hist: &Sudoku) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if hist[i][j] != None && new[i][j] != hist[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn adjust_domains(mut domains: SudokuDomains, sudoku: &Sudoku) -> SudokuDomains {
    for i in 0..9 {
        for j in 0..9 {
            if let Some(v) = sudoku[i][j] {
                domains[i][j] = DomainSet::singelton(v - 1);
            }
        }
    }
    return domains;
}

impl SudokuSolver {
    fn has_no_solution(&self) -> bool {
        if self.current_history().solved == Some(false) {
            return true;
        }
        if self.current_history().prog == 2 {
            for row in self.current_history().domains {
                for cel in row {
                    if cel.is_empty() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn has_multiple_solution(&self) -> bool {
        if self.current_history().prog == 2 {
            for row in self.current_history().domains {
                for cel in row {
                    if !cel.is_singelton() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn is_reducing(&self) -> bool {
        self.current_history().prog < 2
    }

    fn current_history(&self) -> &SudokuHistoryItem {
        &self.history[self.hist_pos]
    }

    fn current_history_mut(&mut self) -> &mut SudokuHistoryItem {
        &mut self.history[self.hist_pos]
    }

    fn start_domain_compute(&mut self) {
        let domains = if self.current_history().prog < 0 {
            sudoku_domains(&self.current_history().sudoku)
        } else {
            self.current_history().domains
        };
        if self.reducing == None && self.current_history().prog < 1 {
            self.reducing = Some(self.current_history().change);
            self.reduce_bridge.send((domains, self.current_history().change));
        }
        if self.minimizing == None && self.current_history().prog < 2 {
            self.minimizing = Some(self.current_history().change);
            self.minimize_bridge.send((domains, self.current_history().change));
        }
    }

    fn history_push(&mut self, mut hist: SudokuHistoryItem) {
        self.change += 1;
        hist.change = self.change;
        if self.hist_pos < self.history.len() - 1 {
            self.history.resize_with(self.hist_pos + 1, || panic!());
        }
        self.history.push(hist);
        self.hist_pos += 1;
    }

    fn smallest_subset(&self, sudoku: &Sudoku) -> Option<usize> {
        let mut min_count = count_domain_values(&sudoku_domains(sudoku));
        let mut best = None;
        for i in (0..self.history.len()).rev() {
            let cnt = count_domain_values(&self.history[i].domains);
            if cnt < min_count && is_sudoku_subset(sudoku, &self.history[i].sudoku) {
                min_count = cnt;
                best = Some(i);
            }
        }
        return best;
    }

    fn history_push_sudoku(&mut self, sudoku: Sudoku) {
        if sudoku != self.current_history().sudoku {
            for i in (0..self.history.len()).rev() {
                if self.history[i].sudoku == sudoku && self.history[i].prog != -1 {
                    self.history_push(self.history[i].clone());
                    return;
                }
            }
            if let Some(idx) = self.smallest_subset(&sudoku) {
                self.history_push(SudokuHistoryItem {
                    sudoku,
                    domains: adjust_domains(self.history[idx].domains, &sudoku),
                    change: 0, prog: 0, solved: None,
                })
            } else {
                self.history_push(SudokuHistoryItem {
                    sudoku,
                    domains: self.current_history().domains,
                    change: 0, prog: -1, solved: None,
                })
            }
        }
    }
}

impl Component for SudokuSolver {
    type Message = SolverMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            history: vec![SudokuHistoryItem::default()],
            hist_pos: 0, change: 0,
            solving: None, reducing: None, minimizing: None,
            solver_bridge: SolvingWorker::bridge(
                ctx.link().callback(|(sol, id)| Self::Message::Solved(sol, id))),
            reduce_bridge: ReducingWorker::bridge(
                ctx.link().callback(|(sol, id)| Self::Message::Reduced(sol, id))),
            minimize_bridge: MinimizingWorker::bridge(
                ctx.link().callback(|(sol, id)| Self::Message::Minimized(sol, id))),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Change(new) => {
                if new != self.current_history().sudoku && self.solving == None {
                    self.history_push_sudoku(new);
                    self.start_domain_compute();
                }
            },
            Self::Message::Solve => {
                if self.solving == None {
                    self.solving = Some(self.change);
                    self.solver_bridge.send((self.current_history().sudoku, self.change));
                }
            },
            Self::Message::Clear => {
                if self.solving == None {
                    self.change += 1;
                    self.history = vec![SudokuHistoryItem::default()];
                    self.hist_pos = 0;
                }
            },
            Self::Message::Solved(res, id) => {
                if self.solving == Some(id) {
                    self.solving = None;
                    if let Some(sol) = res {
                        if sol != self.current_history().sudoku {
                            self.change += 1;
                            self.history_push_sudoku(sol);
                        }
                        self.current_history_mut().solved = Some(true);
                    } else {
                        self.current_history_mut().solved = Some(false);
                    }
                }
            },
            Self::Message::Reduced(sol, id) => {
                if self.reducing == Some(id) {
                    self.reducing = None;
                    for i in (0..self.history.len()).rev() {
                        if self.history[i].change == id && self.history[i].prog < 1 {
                            self.history[i].domains = sol;
                            self.history[i].prog = 1;
                            break;
                        }
                    }
                }
                self.start_domain_compute();
            },
            Self::Message::Minimized(sol, id) => {
                if self.minimizing == Some(id) {
                    self.minimizing = None;
                    for i in (0..self.history.len()).rev() {
                        if self.history[i].change == id && self.history[i].prog < 2 {
                            self.history[i].domains = sol;
                            self.history[i].prog = 2;
                            break;
                        }
                    }
                }
                self.start_domain_compute();
            },
            Self::Message::Undo => {
                if self.hist_pos != 0 {
                    self.hist_pos -= 1;
                    self.start_domain_compute();
                }
            },
            Self::Message::Redo => {
                if self.hist_pos != self.history.len() - 1 {
                    self.hist_pos += 1;
                    self.start_domain_compute();
                }
            }
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="sudoku-solver">
                <SudokuInput
                    sudoku={self.current_history().sudoku}
                    domains={self.current_history().domains}
                    working={self.solving != None}
                    reducing={self.is_reducing()}
                    on_change={ctx.link().callback(|new| Self::Message::Change(new))}
                >
                    <div class="info-text">{
                        if self.has_no_solution() {
                            "no solutions"
                        } else if self.has_multiple_solution() {
                            "multiple solutions"
                        } else {
                            ""
                        }
                    }</div>
                    <div class="buttons">
                        <button
                            onclick={ctx.link().callback(|_| Self::Message::Undo)}
                            disabled={self.hist_pos == 0}
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0z" fill="none"/><path d="M12.5 8c-2.65 0-5.05.99-6.9 2.6L2 7v9h9l-3.62-3.62c1.39-1.16 3.16-1.88 5.12-1.88 3.54 0 6.55 2.31 7.6 5.5l2.37-.78C21.08 11.03 17.15 8 12.5 8z"/></svg>
                        </button>
                        <button
                            onclick={ctx.link().callback(|_| Self::Message::Redo)}
                            disabled={self.hist_pos == self.history.len() - 1}
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px"><path d="M0 0h24v24H0z" fill="none"/><path d="M18.4 10.6C16.55 8.99 14.15 8 11.5 8c-4.65 0-8.58 3.03-9.96 7.22L3.9 16c1.05-3.19 4.05-5.5 7.6-5.5 1.95 0 3.73.72 5.12 1.88L13 16h9V7l-3.6 3.6z"/></svg>
                        </button>
                        <button
                            onclick={ctx.link().callback(|_| Self::Message::Solve)}
                            disabled={self.solving != None || self.current_history().solved != None}
                        >{"Solve"}</button>
                        <button
                            onclick={ctx.link().callback(|_| Self::Message::Clear)}
                        >{"Clear"}</button>
                    </div>
                </SudokuInput>
            </div>
        }
    }
}

