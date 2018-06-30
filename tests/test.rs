extern crate reqwest;
extern crate scroller;

use scroller::ScrollClient;

#[test]
fn should_handle_wrong_host() {
    let url = reqwest::Url::parse("http://localhost:9999").expect("Invalid url");

    assert!(reqwest::get(url.clone()).is_err());

    let client = ScrollClient::new(url, "".into(), None, None, None, false, Vec::new());

    scroller::process(client)
}

#[test]
fn should_handle_wrong_index() {
    let url = reqwest::Url::parse("http://localhost:9200").expect("Invalid url");
    let index = "non-existent-index".into();

    assert!(reqwest::get(url.clone()).is_ok());

    let res = reqwest::get(url.join(index).unwrap());

    assert!(res.is_ok());
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::NotFound);

    let client = ScrollClient::new(url, index.into(), None, None, None, false, Vec::new());

    scroller::process(client)
}

#[test]
fn should_work() {
    let url = reqwest::Url::parse("http://localhost:9200").expect("Invalid url");
    let index = "twitter".into();

    assert!(reqwest::get(url.clone()).is_ok());

    let res = reqwest::get(url.join(index).unwrap());

    assert!(res.is_ok());
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::Ok);

    let client = ScrollClient::new(url, index.into(), None, None, None, false, Vec::new());

    scroller::process(client)
}

#[test]
fn should_handle_nonexisting_output() {}
