extern crate structopt;

use reqwest::{Client, Url};
use elasticsearch::models::*;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde_json;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.",
            raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct ScrollClient {
    /// Url and port of the elastic search host
    host: Url,

    /// Index to scroll
    index: String,

    /// path to a json file containing the query to use (defaults to match_all)
    #[structopt(short = "q", long = "query", parse(from_os_str))]
    query: Option<PathBuf>,

    /// get at most <limit> results
    #[structopt(short = "l", long = "limit")]
    pub limit: Option<usize>,

    /// _source  fields
    source: Vec<String>,
}

impl<'a> IntoIterator for &'a ScrollClient {
    type Item = EsHit;
    type IntoIter = ScrollIter<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        ScrollIter::start_scroll(self)
    }
}

pub struct ScrollIter<'a> {
    host: &'a Url,
    client: Client,
    scroll_id: String,
    hits: Vec<EsHit>,
}

impl<'a> ScrollIter<'a> {
    pub fn start_scroll(scroll_client: &'a ScrollClient) -> Self {
        let client = Client::new();

        let default_query = json!({ "match_all": {}});
        let default_source = vec!["*".into()];

        let _source = if scroll_client.source.is_empty() {
            &default_source
        } else {
            &scroll_client.source
        };

        let query = if let Some(ref path) = scroll_client.query {
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        } else {
            default_query
        };

        let body = json!({
            "query": query,
            "size":  100,
//            "sort": "_uid:asc", // TODO add option to sort on this field (or arbitrary field)
            "_source": _source,
        });

        let path = format!("{}/{}", scroll_client.index.trim_matches('/'), "_search");

        let url = scroll_client.host.join(&path).unwrap();

        let mut res = client
            .get(url)
            .query(&[("scroll", "1m")])
            .json(&body)
            .send()
            .unwrap();

        let es_response = res.json::<EsResponse>().unwrap();

        ScrollIter {
            host: &scroll_client.host,
            client,
            scroll_id: es_response._scroll_id,
            hits: es_response.hits.hits,
        }
    }
}

impl<'a> Iterator for ScrollIter<'a> {
    type Item = EsHit;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.hits.is_empty() {
            let body = json!({
                    "scroll_id": self.scroll_id,
                    "scroll": "1m"
                });

            let url = self.host.join("_search/scroll").unwrap();

            let mut res = self.client.get(url).json(&body).send().unwrap();

            let es_response = res.json::<EsResponse>().unwrap();

            self.scroll_id = es_response._scroll_id;
            self.hits = es_response.hits.hits;

            // todo check timeouts
            // todo check terminated_early bool
        }
        self.hits.pop()
    }
}
