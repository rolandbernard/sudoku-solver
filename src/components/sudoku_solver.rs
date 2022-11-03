use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::components::sudoku_input::SudokuInput;
use crate::solver::domain::DomainSet;
use crate::solver::sudoku::{
    create_problem, default_domains, empty_domains, empty_sudoku, reshape_domains, sudoku_domains,
    Sudoku, SudokuDomains,
};
use crate::workers::{MinimizingWorker, SolvingWorker};

pub enum SolverMessage<const N: usize> {
    Change(Sudoku<N>),
    Solve,
    Clear,
    Solved(Option<Sudoku<N>>, usize),
    Minimized(SudokuDomains<N>, SudokuDomains<N>, usize),
    Undo,
    Redo,
}

#[derive(Clone)]
pub struct SudokuHistoryItem<const N: usize> {
    sudoku: Sudoku<N>,
    domains: SudokuDomains<N>,
    unsure: SudokuDomains<N>,
    change: usize,
    solved: Option<bool>,
}

impl<const N: usize> SudokuHistoryItem<N> {
    fn default() -> Self {
        SudokuHistoryItem {
            sudoku: empty_sudoku(),
            domains: default_domains(),
            unsure: empty_domains(),
            change: 0,
            solved: None,
        }
    }

    fn new(sudoku: Sudoku<N>, domains: SudokuDomains<N>) -> Self {
        let problem = create_problem(&domains);
        let domains = reshape_domains(problem.reduced_domains());
        SudokuHistoryItem {
            sudoku: sudoku,
            domains: domains,
            unsure: domains,
            change: 0,
            solved: None,
        }
    }
}

pub struct SudokuSolver<const N: usize>
where
    Sudoku<N>: Serialize + DeserializeOwned,
    SudokuDomains<N>: Serialize + DeserializeOwned,
{
    history: Vec<SudokuHistoryItem<N>>,
    hist_pos: usize,
    change: usize,
    solving: Option<usize>,
    minimizing: Option<usize>,
    minimize_bridge: Box<dyn Bridge<MinimizingWorker<N>>>,
    solver_bridge: Box<dyn Bridge<SolvingWorker<N>>>,
}

fn count_domain_values<const N: usize>(domains: &SudokuDomains<N>) -> usize {
    let mut res = 0;
    for row in domains {
        for cell in row {
            res += cell.len();
        }
    }
    return res;
}

fn is_sudoku_subset<const N: usize>(new: &Sudoku<N>, hist: &Sudoku<N>) -> bool {
    for i in 0..N {
        for j in 0..N {
            if hist[i][j] != None && new[i][j] != hist[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn adjust_domains<const N: usize>(
    mut domains: SudokuDomains<N>,
    sudoku: &Sudoku<N>,
) -> SudokuDomains<N> {
    for i in 0..N {
        for j in 0..N {
            if let Some(v) = sudoku[i][j] {
                domains[i][j] = DomainSet::singleton(v - 1);
            }
        }
    }
    return domains;
}

impl<const N: usize> SudokuSolver<N>
where
    Sudoku<N>: Serialize + DeserializeOwned,
    SudokuDomains<N>: Serialize + DeserializeOwned,
{
    fn has_no_solution(&self) -> bool {
        if self.current_history().solved == Some(false) {
            return true;
        }
        for row in self.current_history().domains {
            for cel in row {
                if cel.is_empty() {
                    return true;
                }
            }
        }
        return false;
    }

    fn has_multiple_solution(&self) -> bool {
        let SudokuHistoryItem {
            domains, unsure, ..
        } = self.current_history();
        for (&d1, &d2) in domains.iter().flatten().zip(unsure.iter().flatten()) {
            let sure = d1.without_all(d2);
            if !sure.is_empty() && !sure.is_singleton() {
                return true;
            }
        }
        return false;
    }

    fn current_history(&self) -> &SudokuHistoryItem<N> {
        &self.history[self.hist_pos]
    }

    fn current_history_mut(&mut self) -> &mut SudokuHistoryItem<N> {
        &mut self.history[self.hist_pos]
    }

    fn start_domain_compute(&mut self) {
        if self.minimizing == None
            && self
                .current_history()
                .unsure
                .iter()
                .flatten()
                .any(|x| !x.is_empty())
        {
            self.minimizing = Some(self.current_history().change);
            self.minimize_bridge.send((
                self.current_history().domains,
                self.current_history().unsure,
                self.current_history().change,
            ));
        }
    }

    fn history_push(&mut self, mut hist: SudokuHistoryItem<N>) {
        self.change += 1;
        hist.change = self.change;
        if self.hist_pos < self.history.len() - 1 {
            self.history.resize_with(self.hist_pos + 1, || panic!());
        }
        self.history.push(hist);
        self.hist_pos += 1;
    }

    fn smallest_subset(&self, sudoku: &Sudoku<N>) -> Option<usize> {
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

    fn history_push_sudoku(&mut self, sudoku: Sudoku<N>) {
        if sudoku != self.current_history().sudoku {
            for i in (0..self.history.len()).rev() {
                if self.history[i].sudoku == sudoku {
                    self.history_push(self.history[i].clone());
                    return;
                }
            }
            if let Some(idx) = self.smallest_subset(&sudoku) {
                self.history_push(SudokuHistoryItem::new(
                    sudoku,
                    adjust_domains(self.history[idx].domains, &sudoku),
                ));
            } else {
                self.history_push(SudokuHistoryItem::new(sudoku, sudoku_domains(&sudoku)));
            }
        }
    }
}

impl<const N: usize> Component for SudokuSolver<N>
where
    Sudoku<N>: Serialize + DeserializeOwned,
    SudokuDomains<N>: Serialize + DeserializeOwned,
{
    type Message = SolverMessage<N>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            history: vec![SudokuHistoryItem::default()],
            hist_pos: 0,
            change: 0,
            solving: None,
            minimizing: None,
            solver_bridge: SolvingWorker::bridge(
                ctx.link()
                    .callback(|(sol, id)| Self::Message::Solved(sol, id)),
            ),
            minimize_bridge: MinimizingWorker::bridge(
                ctx.link()
                    .callback(|(sol, uns, id)| Self::Message::Minimized(sol, uns, id)),
            ),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Change(new) => {
                if new != self.current_history().sudoku && self.solving == None {
                    self.history_push_sudoku(new);
                    self.start_domain_compute();
                }
            }
            Self::Message::Solve => {
                if self.solving == None {
                    self.solving = Some(self.change);
                    self.solver_bridge
                        .send((self.current_history().sudoku, self.change));
                }
            }
            Self::Message::Clear => {
                if self.solving == None {
                    self.change += 1;
                    self.history = vec![SudokuHistoryItem::default()];
                    self.hist_pos = 0;
                }
            }
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
            }
            Self::Message::Minimized(sol, uns, id) => {
                if self.minimizing == Some(id) {
                    self.minimizing = None;
                    for i in (0..self.history.len()).rev() {
                        if self.history[i].change == id {
                            self.history[i].domains = sol;
                            self.history[i].unsure = uns;
                            break;
                        }
                    }
                }
                self.start_domain_compute();
            }
            Self::Message::Undo => {
                if self.hist_pos != 0 {
                    self.hist_pos -= 1;
                    self.start_domain_compute();
                }
            }
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
                <SudokuInput<N>
                    sudoku={self.current_history().sudoku}
                    domains={self.current_history().domains}
                    unsure={self.current_history().unsure}
                    working={self.solving != None}
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
                </SudokuInput<N>>
            </div>
        }
    }
}
