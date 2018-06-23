extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate structopt;

// TODO try with reqwest and raw post request https://www.elastic.co/guide/en/elasticsearch/guide/1.x/scan-scroll.html
// Updated url https://www.elastic.co/guide/en/elasticsearch/guide/2.x/scroll.html
use structopt::StructOpt;

mod elasticsearch;
use elasticsearch::*;

fn main() {
    let scroll_client = ScrollClient::from_args();

    for item in &scroll_client {
        println!("{:#?}", item);
    }
}
