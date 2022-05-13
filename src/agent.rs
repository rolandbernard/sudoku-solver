
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, Agent, AgentLink};

use crate::solver::sudoku::{Sudoku, create_problem, read_solution};

pub struct Worker {
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInput {
    pub sudoku: Sudoku,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOutput {
    pub sudoku: Sudoku,
}

impl Agent for Worker {
    type Input = WorkerInput;
    type Message = ();
    type Output = WorkerOutput;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let mut sudoku = msg.sudoku;
        let mut prob = create_problem(&sudoku);
        if prob.solve() {
            read_solution(&mut sudoku, &prob);
        }
        let output = Self::Output { sudoku: sudoku };
        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}

