mod unrecoverable;
mod recoverable;
mod propagation;
mod openfile;

fn main() {
    //unrecoverable::demo();
    recoverable::demo();
    propagation::demo();
    openfile::demo();
}