#[macro_use]
extern crate error_chain;
extern crate rayon;

use rayon::prelude::*;

error_chain! {}

fn run() -> Result<()> {
    let v = vec![6, 2, 1, 9, 3, 8, 11];
    let found = v.par_iter().find_any(|&x| x == &9);
    let not_found = v.par_iter().find_any(|&x| x == &0);

    assert!(found.is_some());
    assert!(!not_found.is_some());

    Ok(())
}

quick_main!(run);
