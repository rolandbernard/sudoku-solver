
use yew_agent::{HandlerId, Public, Agent, AgentLink};

use crate::solver::sudoku::*;

pub struct SolvingWorker {
    link: AgentLink<Self>,
}

impl Agent for SolvingWorker {
    type Input = Sudoku;
    type Message = ();
    type Output = Option<Sudoku>;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, sudoku: Self::Input, id: HandlerId) {
        let prob = create_problem(&sudoku);
        let result = prob.find_model()
            .and_then(|v| Some(resize_variables(v)));
        self.link.respond(id, result);
    }

    fn name_of_resource() -> &'static str {
        "worker_solve.js"
    }
}

pub struct ReducingWorker {
    link: AgentLink<Self>,
}

impl Agent for ReducingWorker {
    type Input = Sudoku;
    type Message = ();
    type Output = SudokuDomains;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, sudoku: Self::Input, id: HandlerId) {
        let prob = create_problem(&sudoku);
        let result = resize_domains(prob.reduce_domains());
        self.link.respond(id, result);
    }

    fn name_of_resource() -> &'static str {
        "worker_reduce.js"
    }
}

pub struct MinimizingWorker {
    link: AgentLink<Self>,
}

impl Agent for MinimizingWorker {
    type Input = Sudoku;
    type Message = ();
    type Output = SudokuDomains;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, sudoku: Self::Input, id: HandlerId) {
        let prob = create_problem(&sudoku);
        let result = resize_domains(prob.minimized_domains());
        self.link.respond(id, result);
    }

    fn name_of_resource() -> &'static str {
        "worker_minimize.js"
    }
}

