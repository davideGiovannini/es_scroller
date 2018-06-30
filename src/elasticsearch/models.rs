use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct EsResponse {
    pub _scroll_id: String,
    took: usize,
    timed_out: bool,
    terminated_early: Option<bool>,
    _shards: EsShards,
    pub hits: EsHits,
}

#[derive(Debug, Deserialize)]
pub struct EsShards {
    total: usize,
    successful: usize,
    skipped: usize,
    failed: usize,
}

#[derive(Debug, Deserialize)]
pub struct EsHits {
    pub total: usize,
    max_score: Option<f64>,
    pub hits: Vec<EsHit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EsHit {
    //    _index: String,
    //    _type: String,
    _id: String,
    //    _score: f64,
    //    _routing: Option<String>,
    _source: Value,
}
