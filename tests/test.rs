use scroller::EsError;
use scroller::ScrollerOptions;

use reqwest::blocking::get as GET;

#[test]
fn should_handle_wrong_host() {
    let url = reqwest::Url::parse("http://localhost:9999").expect("Invalid url");

    assert!(GET(url.clone()).is_err());

    let client = ScrollerOptions::new(
        url.clone(),
        "".into(),
        "/dev/null".into(),
        None,
        None,
        false,
        true,
        Vec::new(),
    );

    let result = scroller::process(&client);

    assert!(result.is_err());
    match result.unwrap_err() {
        EsError::HostUnreachable(host_url) => assert_eq!(url, host_url),
        _ => panic!("tested function returned the wrong error"),
    }
}

#[test]
fn should_handle_wrong_index() {
    let url = reqwest::Url::parse("http://localhost:9200").expect("Invalid url");
    let index = "non-existent-index";

    assert!(GET(url.clone()).is_ok());

    let res = GET(url.join(index).unwrap());

    assert!(res.is_ok());
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::NOT_FOUND);

    let client = ScrollerOptions::new(
        url,
        index.into(),
        "/dev/null".into(),
        None,
        None,
        false,
        true,
        Vec::new(),
    );

    let result = scroller::process(&client);

    assert!(result.is_err());

    match result.unwrap_err() {
        EsError::IndexNotFound(e_index, ..) => assert_eq!(e_index, index.into()),
        _ => panic!("tested function returned the wrong error"),
    }
}

#[test]
fn should_work() {
    let url = reqwest::Url::parse("http://localhost:9200").expect("Invalid url");
    let index = "twitter";

    assert!(GET(url.clone()).is_ok());

    let res = GET(url.join(index).unwrap());

    assert!(res.is_ok());
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::OK);

    let client = ScrollerOptions::new(
        url,
        index.into(),
        "/dev/null".into(),
        None,
        None,
        false,
        true,
        Vec::new(),
    );

    let result = scroller::process(&client);

    assert!(result.is_ok())
}

#[test]
fn should_handle_timeout() {
    unimplemented!()
}

#[test]
fn should_handle_nonexisting_output() {
    unimplemented!()
}

#[test]
fn should_handle_nonexisting_input_query() {
    unimplemented!()
}

#[test]
fn should_handle_malformed_json_input_query() {
    unimplemented!()
}
