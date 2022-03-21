use isahc::Body;

pub async fn get(url: &str) -> Result<isahc::Response<Body>, isahc::Response<Body>> {
    let requestresponse = isahc::get(url).unwrap();
    if requestresponse.status().is_success() == true {
        return Ok(requestresponse);
    } else {
        return Err(requestresponse);
    }
}

pub async fn post(url: &str, body: Body) -> Result<isahc::Response<Body>, isahc::Response<Body>> {
    let requestresponse = isahc::post(url, body).unwrap();
    if requestresponse.status().is_success() == true {
        return Ok(requestresponse);
    } else {
        return Err(requestresponse);
    }
}