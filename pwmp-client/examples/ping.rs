use std::net::{Ipv4Addr, SocketAddrV4};

use pwmp_client::PwmpClient;
use pwmp_types::mac::Mac;

fn main() {
    let mut client = PwmpClient::new(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 55300),
        Mac::new(1, 2, 3, 4, 5, 6),
    )
    .unwrap();

    assert!(client.ping());
}
