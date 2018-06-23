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

mod elasticsearch;
use elasticsearch::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct App{

}



fn main() {

    let scroll_client = ScrollClient::new(
        "http://localhost:9200".to_string(),
        "atoka-companies-latest".to_string(),
        "".to_string(),
        None
    );

    for item in &scroll_client{
        println!("{:#?}", item);
    }
}
