
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

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

impl SudokuSolver {
    fn has_no_solution(&self) -> bool {
        if self.solved == Some(false) {
            return true;
        }
        if self.domain_change >= 2*self.change {
            for row in self.domains {
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
        if self.domain_change == 1 + 2*self.change {
            for row in self.domains {
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
        self.domain_change < 1 + 2*self.change
    }
}

impl Component for SudokuSolver {
    type Message = SolverMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            sudoku: empty_sudoku(),
            domains: default_domains(),
            change: 0, domain_change: 1,
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
                if new != self.sudoku && self.solving == None {
                    self.change += 1;
                    self.sudoku = new;
                    self.solved = None;
                    if self.reducing == None {
                        self.reducing = Some(self.change);
                        self.reduce_bridge.send((sudoku_domains(&self.sudoku), self.change));
                    }
                    if self.minimizing == None {
                        self.minimizing = Some(self.change);
                        self.minimize_bridge.send((sudoku_domains(&self.sudoku), self.change));
                    }
                }
            },
            Self::Message::Solve => {
                if self.solving == None {
                    self.solving = Some(self.change);
                    self.solver_bridge.send((self.sudoku, self.change));
                }
            },
            Self::Message::Clear => {
                if self.solving == None {
                    self.change += 1;
                    self.sudoku = empty_sudoku();
                    self.domains = default_domains();
                    self.domain_change = 1 + 2*self.change;
                    self.solved = None;
                }
            },
            Self::Message::Solved(res, id) => {
                if self.solving == Some(id) {
                    self.solving = None;
                    if let Some(sol) = res {
                        self.change += 1;
                        self.sudoku = sol;
                        self.solved = Some(true);
                        self.domains = sudoku_domains(&self.sudoku);
                        self.domain_change = 1 + 2*self.change;
                    } else {
                        self.solved = Some(false);
                    }
                }
            },
            Self::Message::Reduced(sol, id) => {
                if self.reducing == Some(id) {
                    self.reducing = None;
                    if self.domain_change < 2*id {
                        self.domains = sol;
                        self.domain_change = 2*id;
                    }
                }
                if self.domain_change < 2*self.change {
                    self.reducing = Some(self.change);
                    self.reduce_bridge.send((sudoku_domains(&self.sudoku), self.change));
                }
            },
            Self::Message::Minimized(sol, id) => {
                if self.minimizing == Some(id) {
                    self.minimizing = None;
                    if self.domain_change < 1 + 2*id {
                        self.domains = sol;
                        self.domain_change = 1 + 2*id;
                    }
                }
                if self.domain_change < 1 + 2*self.change {
                    self.minimizing = Some(self.change);
                    self.minimize_bridge.send((sudoku_domains(&self.sudoku), self.change));
                }
            },
        }
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="sudoku-solver">
                <div class="status-row">
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
                            onclick={ctx.link().callback(|_| Self::Message::Solve)}
                            disabled={self.solving != None || self.solved != None}
                        >{"Solve"}</button>
                        <button
                            onclick={ctx.link().callback(|_| Self::Message::Clear)}
                        >{"Clear"}</button>
                    </div>
                </div>
                <SudokuInput
                    sudoku={self.sudoku}
                    domains={self.domains}
                    working={self.solving != None}
                    reducing={self.is_reducing()}
                    on_change={ctx.link().callback(|new| Self::Message::Change(new))}
                />
            </div>
        }
    }
}

