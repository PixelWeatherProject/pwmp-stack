use pwmp_client::PwmpClient;
use pwmp_types::mac::Mac;

fn main() {
    let mut client = PwmpClient::new("127.0.0.1:55300", Mac::new(1, 2, 3, 4, 5, 6)).unwrap();

    client
        .post_measurements(Default::default(), 100, None, Default::default())
        .unwrap();
}
