//! [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
//!
//! These codes allow command line applications to show colored or styled text in most terminals.
//! Advanced commands can move the cursor or clear the screen.

use aoc::utils::common::*;
use aoc::utils::launcher::*;
use std::env::args;



fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let args = parse_usize(&arg);
            if args.len() == 0 {
                (None, None)
            } else if args.len() == 1 {
                (Some(args[0]), None)
            } else {
                (Some(args[0]), Some(args[1]))
            }
        }
        None => (None, None),
    };
    launch(year, day);
}


