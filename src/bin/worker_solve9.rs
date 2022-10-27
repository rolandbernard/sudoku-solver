
use yew_agent::Threaded;

use sudoku::workers::SolvingWorker;

fn main() {
    SolvingWorker::<9>::register();
}

