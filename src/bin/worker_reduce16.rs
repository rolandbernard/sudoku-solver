
use yew_agent::Threaded;

use sudoku::workers::ReducingWorker;

fn main() {
    ReducingWorker::<16>::register();
}

