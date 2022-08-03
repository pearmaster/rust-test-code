


fn main() {
    let parsed = json::parse(r#"{"value":100}"#);
    let payload = parsed.unwrap();
    println!("Hello, world: {}", payload["value"]);

    let obj = json::object!{"key":"value"};
    println!("object: {}", json::stringify(obj));
}
