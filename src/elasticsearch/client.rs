use crate::elasticsearch::models::*;
use reqwest::{Client, Url};

use reqwest::StatusCode;
use serde_json::json;

use std::fs::File;
use std::io::Read;

use crate::args::ScrollerOptions;
use crate::elasticsearch::errors::EsError;

pub struct ScrollClient<'a> {
    host: &'a Url,
    client: Client,
    scroll_id: String,
    results_count: usize,
    hits: Vec<EsHit>,
}

impl ScrollClient<'_> {
    pub fn new(options: &ScrollerOptions) -> Result<ScrollClient, EsError> {
        let client = Client::new();

        let default_query = json!({ "match_all": {}});
        let default_source = vec!["*".into()];

        let _source = if options.source.is_empty() {
            &default_source
        } else {
            &options.source
        };

        let query = if let Some(ref path) = options.query {
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

        let path = format!("{}/{}", options.index.trim_matches('/'), "_search");

        let url = options.host.join(&path).unwrap();

        let res = client
            .get(url)
            .query(&[("scroll", "1m")])
            .json(&body)
            .send();

        let mut res = res.map_err(|_| EsError::HostUnreachable(options.host.clone()))?;

        if res.status() == StatusCode::NOT_FOUND {
            let suggested = suggest_correct_index_name(&client, &options.host, &options.index);
            return Err(EsError::IndexNotFound(
                options.index.as_str().into(),
                suggested,
            ));
        }

        let es_response = res.json::<EsResponse>().unwrap();
        Ok(ScrollClient {
            host: &options.host,
            client,
            results_count: es_response.hits.total,
            scroll_id: es_response._scroll_id,
            hits: es_response.hits.hits,
        })
    }
}

impl<'a> Iterator for ScrollClient<'a> {
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

impl<'a> Drop for ScrollClient<'a> {
    fn drop(&mut self) {
        // Delete the scroll

        let url = self.host.join("_search/scroll").unwrap();
        let body = json!({
            "scoll_id": self.scroll_id
        });
        self.client.delete(url).json(&body).send().unwrap();
    }
}

const MAX_EDIT_DISTANCE_FOR_SUGGESTER: usize = 15;

fn suggest_correct_index_name(client: &Client, host: &Url, index_name: &str) -> Option<Index> {
    let mut res = client
        .get(host.join(&"_cat/indices?format=json").ok()?)
        .send()
        .ok()?;
    let mut names = res.json::<Vec<Index>>().ok()?;

    use edit_distance::edit_distance;
    names.sort_by_key(|index| edit_distance(index_name, &index.name));

    let first = names
        .first()
        .filter(|index| edit_distance(index_name, &index.name) <= MAX_EDIT_DISTANCE_FOR_SUGGESTER);

    first.cloned()
}
