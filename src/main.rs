use cloudevents::{message::BinaryDeserializer, EventBuilder, EventBuilderV10};
use bytes::Bytes;
use http;
use http::Request;
use wasi_experimental_http;

fn main() {
    const POSTMAN_ECHO_POST_URL: &str = "https://postman-echo.com/post";

    loop {
        let builder = http::request::Builder::new()
            .uri(POSTMAN_ECHO_POST_URL)
            .method(http::Method::POST);

        let event = EventBuilderV10::new()
            .id("0001".to_string())
            .source("http://localhost/".to_string())
            .ty("test_event.test_application".to_string())
            .build()
            .unwrap();

        let request: Request<Option<Bytes>> =
            BinaryDeserializer::deserialize_binary(event, builder).unwrap();

        println!("{:?}", request);

        let mut response = wasi_experimental_http::request(request).expect("cannot make request");
        let response_body = response.body_read_all().unwrap();
        let response_text = std::str::from_utf8(&response_body).unwrap().to_string();

        println!("{}", response.status_code);
        println!("{}", response_text);

        std::thread::sleep(std::time::Duration::new(15, 0));
    }
}
