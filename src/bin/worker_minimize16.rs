use yew_agent::Threaded;

use sudoku::workers::MinimizingWorker;

fn main() {
    MinimizingWorker::<16>::register();
}
