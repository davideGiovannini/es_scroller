use serde_json::Value;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct EsResponse {
    _scroll_id: String,
    took: usize,
    timed_out: bool,
    terminated_early: Option<bool>,
    _shards: EsShards,
    hits: EsHits,
}

#[derive(Debug, Deserialize)]
struct EsShards {
    total: usize,
    successful: usize,
    skipped: usize,
    failed: usize,
}

#[derive(Debug, Deserialize)]
struct EsHits {
    total: usize,
    max_score: f64,
    hits: Vec<EsHit>,
}

#[derive(Debug, Deserialize)]
pub struct EsHit {
    _index: String,
    _type: String,
    _id: String,
    _score: f64,
    _routing: Option<String>,
    _source: Value,
}

pub struct ScrollClient {
    host: String,
    index: String,
    query: String,
    source: Option<String>,
}

impl ScrollClient {
    pub fn new(host: String, index: String, query: String, source: Option<String>) -> Self {
        ScrollClient {
            host,
            index,
            query,
            source,
        }
    }
}

impl<'a> IntoIterator for &'a ScrollClient {
    type Item = EsHit;
    type IntoIter = ScrollIter<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        ScrollIter::start_scroll(self)
    }
}

pub struct ScrollIter<'a> {
    scroll_client: &'a ScrollClient,
    client: Client,
    scroll_id: String,
    hits: Vec<EsHit>,
}

impl<'a> ScrollIter<'a> {
    fn start_scroll(scroll_client: &'a ScrollClient) -> Self {
        let client = Client::new();

        let body = json!({
        "query": { "match_all": {}},
        "size":  1
//        "_source": "acheneID"
    });

        let mut res = client
            .get(&format!(
                "{}/{}/_search",
                scroll_client.host, scroll_client.index
            ))
            .query(&[("scroll", "1m")])
            .json(&body)
            .send()
            .unwrap();

        let es_respones = res.json::<EsResponse>().unwrap();

        ScrollIter {
            scroll_client,
            client,
            scroll_id: es_respones._scroll_id,
            hits: es_respones.hits.hits,
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

            let mut res = self.client
                .get(&format!("{}/_search/scroll", self.scroll_client.host))
                .json(&body)
                .send()
                .unwrap();

            let es_respones = res.json::<EsResponse>().unwrap();

            self.scroll_id = es_respones._scroll_id;
            self.hits = es_respones.hits.hits;

            // todo check timeouts
            // todo check terminated_early bool
        }
        self.hits.pop()
    }
}
