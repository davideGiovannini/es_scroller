use reqwest::{Url, UrlError};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A simple rust client to perform scroll search requests to an ElasticSearch cluster.",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct ScrollerOptions {
    /// ElasticSearch host, protocol and/or port can be omitted if they are respectively "http" or ":9200"
    #[structopt(parse(try_from_str = parse_url))]
    pub host: Url,

    /// Index to scroll
    pub index: String,

    /// path of the output jsonl file (use - to output to stdout instead)
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// path to a json file containing the query to use (defaults to match_all)
    #[structopt(short = "q", long = "query", parse(from_os_str))]
    pub query: Option<PathBuf>,

    /// get at most <limit> results
    #[structopt(short = "l", long = "limit")]
    pub limit: Option<usize>,

    /// pretty print output
    #[structopt(short = "p", long = "pretty")]
    pub pretty: bool,

    /// hide the progressbar
    #[structopt(short = "s", long = "silent")]
    pub silent: bool,

    /// _source  fields
    pub source: Vec<String>,
}

impl ScrollerOptions {
    pub fn new(
        host: Url,
        index: String,
        output: PathBuf,
        query: Option<PathBuf>,
        limit: Option<usize>,
        pretty: bool,
        silent: bool,
        source: Vec<String>,
    ) -> Self {
        ScrollerOptions {
            host,
            index,
            output,
            query,
            limit,
            pretty,
            silent,
            source,
        }
    }
}

fn parse_url(src: &str) -> Result<Url, UrlError> {
    let url = match Url::parse(src) {
        Err(UrlError::RelativeUrlWithoutBase) => {
            let a = format!("{}:9200", src);
            Url::parse(&a)
        }
        case => case,
    }?;

    if url.cannot_be_a_base() {
        Url::parse(&format!("http://{}", url))
    } else {
        Ok(url)
    }
}
