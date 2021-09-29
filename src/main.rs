use cloudevents::{message::*, Event, EventBuilder, EventBuilderV10, binding::http::*};
use bytes::Bytes;
use http;
use http::Request;
use wasi_experimental_http::Response;
use std::env;

fn main() {
    env_logger::init();
    let echo_service_url = env::var("ECHO_SERVICE_URL").unwrap();

    loop {
        let builder = http::request::Builder::new()
            .uri(echo_service_url.clone())
            .method(http::Method::POST);

        let input = EventBuilderV10::new()
            .id("0001".to_string())
            .source("http://localhost/".to_string())
            .ty("test_event.test_application".to_string())
            .build()
            .unwrap();

        let request: Request<Option<Bytes>> =
            BinaryDeserializer::deserialize_binary(input, builder).unwrap();

        log::info!("Request: {:?}", request);

        let mut response = wasi_experimental_http::request(request).expect("cannot make request");
        let response_body = response.body_read_all().unwrap();
        let response_text = std::str::from_utf8(&response_body).unwrap().to_string();

        log::info!("{}", response.status_code);
        log::info!("Response: {:?}", response_text);

        // If the echo server returns event in response, this can be used to parse it
        //let output = response_to_event(response);
        //log::info!("Response: {:?}", output);

        std::thread::sleep(std::time::Duration::new(15, 0));
    }
}


/// Method to transform an incoming [`Response`] to [`Event`].
pub fn response_to_event(mut res: Response) -> Result<Event> {
    let headers = res.headers_get_all().unwrap();
    let body = res.body_read_all().unwrap();

    to_event(&headers, body)
}