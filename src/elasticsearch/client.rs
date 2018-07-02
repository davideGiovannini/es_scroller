extern crate structopt;

use reqwest::{Client, Url};
use elasticsearch::models::*;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use serde_json;
use reqwest::StatusCode;

use elasticsearch::errors::EsError;

#[derive(Debug, StructOpt)]
#[structopt(about = "An example of StructOpt usage.",
            raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct ScrollClient {
    /// Url and port of the elastic search host
    host: Url,

    /// Index to scroll
    pub index: String,

    /// path of the output jsonl file (only if not using --stream)
    #[structopt(required_unless = "stream", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// path to a json file containing the query to use (defaults to match_all)
    #[structopt(short = "q", long = "query", parse(from_os_str))]
    query: Option<PathBuf>,

    /// get at most <limit> results
    #[structopt(short = "l", long = "limit")]
    pub limit: Option<usize>,

    /// pretty print output
    #[structopt(short = "p", long = "pretty")]
    pub pretty: bool,

    /// hide the progressbar
    #[structopt(short = "s", long = "silent")]
    pub silent: bool,

    /// Stream output to stdout
    #[structopt(long = "stream")]
    pub stream: bool,

    /// _source  fields
    source: Vec<String>,
}

impl ScrollClient {
    pub fn new(
        host: Url,
        index: String,
        output: Option<PathBuf>,
        query: Option<PathBuf>,
        limit: Option<usize>,
        pretty: bool,
        silent: bool,
        stream: bool,
        source: Vec<String>,
    ) -> Self {
        ScrollClient {
            host,
            index,
            output,
            query,
            limit,
            pretty,
            silent,
            stream,
            source,
        }
    }
    pub fn start_scroll(&self) -> Result<ScrollIter, EsError> {
        let client = Client::new();

        let default_query = json!({ "match_all": {}});
        let default_source = vec!["*".into()];

        let _source = if self.source.is_empty() {
            &default_source
        } else {
            &self.source
        };

        let query = if let Some(ref path) = self.query {
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        } else {
            default_query
        };

        let body = json!({
    "query": query,
    "size":  1000,
    "sort": &["_doc"], // TODO add option to sort on this field (or arbitrary field)
    "_source": _source,
    });

        let path = format!("{}/{}", self.index.trim_matches('/'), "_search");

        let url = self.host.join(&path).unwrap();

        let res = client
            .get(url)
            .query(&[("scroll", "1m")])
            .json(&body)
            .send();

        let mut res = res.map_err(|_| EsError::HostUnreachable)?;

        if res.status() == StatusCode::NotFound {
            return Err(EsError::IndexNotFound);
        }

        let es_response = res.json::<EsResponse>().unwrap();
        Ok(ScrollIter {
            host: &self.host,
            client,
            results_count: es_response.hits.total,
            scroll_id: es_response._scroll_id,
            hits: es_response.hits.hits,
        })
    }
}

pub struct ScrollIter<'a> {
    host: &'a Url,
    client: Client,
    scroll_id: String,
    results_count: usize,
    hits: Vec<EsHit>,
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.results_count, None)
    }
}
