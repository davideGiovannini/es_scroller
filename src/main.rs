extern crate scroller;
extern crate structopt;

use scroller::{EsError, ScrollerOptions};
use structopt::StructOpt;

fn main() -> Result<(), EsError> {
    let scroll_options = ScrollerOptions::from_args();
    scroller::process(&scroll_options)
}
