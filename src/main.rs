extern crate scroller;
extern crate structopt;

use structopt::StructOpt;
use scroller::ScrollClient;

fn main() {
    let scroll_client = ScrollClient::from_args();
    scroller::process(scroll_client)
}
