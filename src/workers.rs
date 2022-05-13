
use yew_agent::{HandlerId, Public, Agent, AgentLink};

use crate::solver::{sudoku::{*, self}, solver::*};

pub struct SolvingWorker {
    link: AgentLink<Self>,
}

impl Agent for SolvingWorker {
    type Input = Sudoku;
    type Message = ();
    type Output = Sudoku;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, mut sudoku: Self::Input, id: HandlerId) {
        let mut prob = create_problem(&sudoku);
        if prob.solve() {
            read_solution(&mut sudoku, &prob);
        }
        self.link.respond(id, sudoku);
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}

