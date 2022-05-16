
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::solver::sudoku::{empty_sudoku, default_domains, Sudoku, SudokuDomains};
use crate::workers::{SolvingWorker, ReducingWorker, MinimizingWorker};
use crate::components::sudoku_input::SudokuInput;

pub enum SolverMessage {
    Change(Sudoku),
    Solve,
    Clear,
    Solved(Option<Sudoku>, usize),
    Reduced(SudokuDomains, usize),
    Minimized(SudokuDomains, usize),
}

pub struct SudokuSolver {
    sudoku: Sudoku,
    domains: SudokuDomains,
    domain_change: usize,
    change: usize,
    solved: Option<bool>,
    solving: Option<usize>,
    reducing: Option<usize>,
    minimizing: Option<usize>,
    minimize_bridge: Box<dyn Bridge<MinimizingWorker>>,
    reduce_bridge: Box<dyn Bridge<ReducingWorker>>,
    solver_bridge: Box<dyn Bridge<SolvingWorker>>,
}

impl Component for SudokuSolver {
    type Message = SolverMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            sudoku: empty_sudoku(),
            domains: default_domains(),
            change: 0, domain_change: 0,
            solved: None, solving: None,
            reducing: None, minimizing: None,
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
                if new != self.sudoku {
                    self.change += 1;
                    self.sudoku = new;
                    self.solved = None;
                    if self.reducing == None {
                        self.reducing = Some(self.change);
                        self.reduce_bridge.send((self.sudoku, self.change));
                    }
                    if self.minimizing == None {
                        self.minimizing = Some(self.change);
                        self.minimize_bridge.send((self.sudoku, self.change));
                    }
                }
            },
            Self::Message::Solve => {
                self.solving = Some(self.change);
                self.solver_bridge.send((self.sudoku, self.change));
            },
            Self::Message::Clear => {
                self.change += 1;
                self.sudoku = empty_sudoku();
                self.domains = default_domains();
                self.domain_change = self.change;
                self.solved = None;
            },
            Self::Message::Solved(res, id) => {
                if self.solving == Some(id) {
                    if let Some(sol) = res {
                        self.sudoku = sol;
                        self.solving = None;
                        self.solved = Some(true);
                    } else {
                        self.solved = Some(false);
                    }
                }
            },
            Self::Message::Reduced(sol, id) => {
                if self.reducing == Some(id) && self.domain_change < id {
                    self.domains = sol;
                    self.domain_change = id;
                    self.reducing = None;
                }
                if self.domain_change < self.change {
                    self.reducing = Some(self.change);
                    self.reduce_bridge.send((self.sudoku, self.change));
                }
            },
            Self::Message::Minimized(sol, id) => {
                if self.minimizing == Some(id) && self.domain_change <= id {
                    self.domains = sol;
                    self.domain_change = id;
                    self.reducing = None;
                    self.minimizing = None;
                }
                if id < self.change {
                    self.minimizing = Some(self.change);
                    self.minimize_bridge.send((self.sudoku, self.change));
                }
            },
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="sudoku-solver">
                <SudokuInput
                    sudoku={self.sudoku}
                    domains={self.domains}
                    working={self.solving != None}
                    reducing={self.reducing != None || self.minimizing != None}
                    on_change={ctx.link().callback(|new| Self::Message::Change(new))}
                />
                <div class="button-row">
                    <button
                        onclick={ctx.link().callback(|_| Self::Message::Solve)}
                        disabled={self.solving != None || self.solved != None}
                    >{"Solve"}</button>
                    <button
                        onclick={ctx.link().callback(|_| Self::Message::Clear)}
                    >{"Clear"}</button>
                </div>
            </div>
        }
    }
}

