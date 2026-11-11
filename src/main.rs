use crate::loader::{Context, load_context};

pub mod graph;
pub mod loader;

fn main() {
    let _: Context = load_context();
}
