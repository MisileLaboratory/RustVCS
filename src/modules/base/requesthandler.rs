use reqwest::header::HeaderMap;

use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};

pub fn get_default_headers(token: String) -> HeaderMap {
    let mut requestresponse = HeaderMap::new();
    requestresponse.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
    requestresponse.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    requestresponse.insert(AUTHORIZATION, format!("token {}", token).parse().unwrap());
    return requestresponse;
}
