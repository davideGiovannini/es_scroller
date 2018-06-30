extern crate scroller;
extern crate structopt;

use structopt::StructOpt;
use scroller::{EsError, ScrollClient};

fn main() -> Result<(), EsError> {
    let scroll_client = ScrollClient::from_args();
    scroller::process(scroll_client)
}
