use cloudevents::{
    message::BinaryDeserializer, EventBuilder, EventBuilderV10,
};
use http;
use http::Request;
use wasi_experimental_http;

fn main() {
    //const POSTMAN_ECHO_PAYLOAD: &[u8] = b"I'm not superstitious, but I am a little stitious.";
    const POSTMAN_ECHO_POST_URL: &str = "https://postman-echo.com/post";

    let builder = http::request::Builder::new().uri(POSTMAN_ECHO_POST_URL);

    //let request_body = Bytes::from(POSTMAN_ECHO_PAYLOAD);

    let event = EventBuilderV10::new()
        .id("0001".to_string())
        .source("http://localhost/".to_string())
        .ty("test_event.test_application".to_string())
        .build()
        .unwrap();

    let request: Request<Option<bytes::Bytes>> =
        BinaryDeserializer::deserialize_binary(event, builder).unwrap();

    // let request = http::request::Builder::new()
    //     .method(http::Method::POST)
    //     .uri(POSTMAN_ECHO_POST_URL)
    //     .header("Content-Type", "text/plain")
    //     .body(Some(request_body))
    //     .expect("cannot construct request");

    let mut response = wasi_experimental_http::request(request).expect("cannot make request");
    let response_body = response.body_read_all().unwrap();
    let response_text = std::str::from_utf8(&response_body).unwrap().to_string();

    println!("{}", response.status_code);
    println!("{}", response_text);
}
