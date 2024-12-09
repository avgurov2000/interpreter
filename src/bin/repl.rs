use interpreter::repl::start;
use std::io;

fn main() {
    start(io::stdin().lock(), io::stdout());
}
