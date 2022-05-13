
use yew_agent::Threaded;
use sudoku::workers::SolvingWorker;

fn main() {
    SolvingWorker::register();
}

