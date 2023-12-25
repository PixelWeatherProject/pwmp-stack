use pwmp_types::{request::Request, Message};

#[test]
pub fn gen_verify() {
    let message = Message::Request(Request::Ping);
    let raw = message.clone().to_raw();
    let parsed = Message::from_raw(&raw).unwrap();

    assert_eq!(message, parsed);
}
