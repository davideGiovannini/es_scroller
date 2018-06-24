extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate structopt;

use std::io::stdout;
use std::io::Write;
use serde_json::Error;

// TODO try with reqwest and raw post request https://www.elastic.co/guide/en/elasticsearch/guide/1.x/scan-scroll.html
// Updated url https://www.elastic.co/guide/en/elasticsearch/guide/2.x/scroll.html
use structopt::StructOpt;

mod elasticsearch;
use elasticsearch::*;

fn main() {
    let scroll_client = ScrollClient::from_args();

    let print_function = if scroll_client.pretty {
        serde_json::to_string_pretty
    } else {
        serde_json::to_string
    };

    if let Some(limit) = scroll_client.limit {
        process_elements(scroll_client.into_iter().take(limit), print_function)
    } else {
        process_elements(scroll_client.into_iter(), print_function)
    };
}

fn process_elements<I>(scroll: I, print_function: fn(&I::Item) -> Result<String, Error>)
where
    I: std::iter::Iterator,
    I::Item: serde::Serialize,
{
    let stdout = stdout();
    let mut stdout_lock = stdout.lock();

    for item in scroll {
        let string = print_function(&item).unwrap();
        writeln!(&mut stdout_lock, "{}", &string).unwrap();
    }
}
