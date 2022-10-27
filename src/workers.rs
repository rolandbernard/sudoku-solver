
use serde::{Serialize, de::DeserializeOwned};
use yew_agent::{HandlerId, Public, Agent, AgentLink};

use crate::solver::sudoku::*;

pub struct SolvingWorker<const N: usize> 
    where Sudoku<N>: Serialize + DeserializeOwned
{
    link: AgentLink<Self>,
}

impl<const N: usize> Agent for SolvingWorker<N> 
    where Sudoku<N>: Serialize + DeserializeOwned
{
    type Input = (Sudoku<N>, usize);
    type Message = ();
    type Output = (Option<Sudoku<N>>, usize);
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

pub struct ReducingWorker<const N: usize> where Sudoku<N>: Serialize + DeserializeOwned,
SudokuDomains<N>: Serialize + DeserializeOwned
{
    link: AgentLink<Self>,
}

impl<const N: usize> Agent for ReducingWorker<N> where Sudoku<N>: Serialize +DeserializeOwned, SudokuDomains<N>: Serialize + DeserializeOwned{
    type Input = (SudokuDomains<N>, usize);
    type Message = ();
    type Output = (SudokuDomains<N>, usize);
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

pub struct MinimizingWorker<const N: usize>
where SudokuDomains<N>: Serialize + DeserializeOwned
{
    link: AgentLink<Self>,
}

impl<const N: usize> Agent for MinimizingWorker<N> 
where SudokuDomains<N>: Serialize + DeserializeOwned
{
    type Input = (SudokuDomains<N>, usize);
    type Message = ();
    type Output = (SudokuDomains<N>, usize);
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

