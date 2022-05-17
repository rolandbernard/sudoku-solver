
use yew_agent::{HandlerId, Public, Agent, AgentLink};

use crate::solver::sudoku::*;

pub struct SolvingWorker {
    link: AgentLink<Self>,
}

impl Agent for SolvingWorker {
    type Input = (Sudoku, usize);
    type Message = ();
    type Output = (Option<Sudoku>, usize);
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let (sudoku, change) = msg;
        let prob = create_problem(&sudoku_domains(&sudoku));
        let result = prob.find_model()
            .and_then(|v| Some(resize_variables(v)));
        self.link.respond(id, (result, change));
    }

    fn name_of_resource() -> &'static str {
        "worker_solve.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}

pub struct ReducingWorker {
    link: AgentLink<Self>,
}

impl Agent for ReducingWorker {
    type Input = (SudokuDomains, usize);
    type Message = ();
    type Output = (SudokuDomains, usize);
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let (sudoku, change) = msg;
        let prob = create_problem(&sudoku);
        let result = resize_domains(prob.reduce_domains());
        self.link.respond(id, (result, change));
    }

    fn name_of_resource() -> &'static str {
        "worker_reduce.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}

pub struct MinimizingWorker {
    link: AgentLink<Self>,
}

impl Agent for MinimizingWorker {
    type Input = (SudokuDomains, usize);
    type Message = ();
    type Output = (SudokuDomains, usize);
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let (sudoku, change) = msg;
        let prob = create_problem(&sudoku);
        let result = resize_domains(prob.minimized_domains());
        self.link.respond(id, (result, change));
    }

    fn name_of_resource() -> &'static str {
        "worker_minimize.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}

