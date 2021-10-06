use cloudevents::{EventBuilder, EventBuilderV10, binding::http::*};
use bytes::Bytes;
use http;
use http::Request;
use std::env;
use std::convert::TryFrom;

fn main() {
    env_logger::init();
    let echo_service_url = env::var("ECHO_SERVICE_URL").unwrap();

    loop {
        let input = EventBuilderV10::new()
            .id("0001".to_string())
            .source("http://localhost/".to_string())
            .ty("test_event.test_application".to_string())
            .build()
            .unwrap();

        log::info!("Event: {}", input);

        let mut request: Request<Option<Bytes>> = Request::try_from(input).unwrap();
        let (mut parts, body) = request.into_parts();
        parts.uri = TryFrom::try_from(echo_service_url.clone()).unwrap();
        parts.method = http::Method::POST;
        request = Request::from_parts(parts, body);


        log::info!("Request: {:?}", request);

        let mut response = wasi_experimental_http::request(request).expect("cannot make request");
        let response_body = response.body_read_all().unwrap();
        let response_text = std::str::from_utf8(&response_body).unwrap().to_string();
        let headers = response.headers_get_all().unwrap();

        log::info!("{}", response.status_code);
        log::info!("Response: {:?} {:?}", headers, response_text);

        match to_event(&headers, response_body) {
            Ok(event) => {
                log::info!("Event {}", event)
            }
            Err(_) => {}
        }

        std::thread::sleep(std::time::Duration::new(15, 0));
    }
}