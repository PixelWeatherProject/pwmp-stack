use std::str::FromStr;

use pwmp_types::mac::{Mac, MacParseError};

#[test]
fn mac_nth_octet() {
    let mac = Mac::new(1, 2, 3, 4, 5, 6);

    assert_eq!(mac.nth_octet(0), 1);
    assert_eq!(mac.nth_octet(1), 2);
    assert_eq!(mac.nth_octet(2), 3);
    assert_eq!(mac.nth_octet(3), 4);
    assert_eq!(mac.nth_octet(4), 5);
    assert_eq!(mac.nth_octet(5), 6);
}

#[test]
fn mac_to_string() {
    assert_eq!(&Mac::new(1, 2, 3, 4, 5, 6).to_string(), "01:02:03:04:05:06");
    assert_eq!(
        &Mac::new(42, 96, 88, 120, 255, 0).to_string(),
        "2A:60:58:78:FF:00"
    );
}

#[test]
fn mac_from_string() {
    assert_eq!(
        Mac::from_str("01:02:03:04:05:06").unwrap(),
        Mac::new(1, 2, 3, 4, 5, 6)
    );
    assert_eq!(
        Mac::from_str("2A:60:58:78:FF:00").unwrap(),
        Mac::new(42, 96, 88, 120, 255, 0)
    );
}

#[test]
fn mac_from_string_too_long() {
    assert_eq!(Mac::from_str("01:02:03:04:05:06x"), Err(MacParseError));
    assert_eq!(Mac::from_str("hello, : : world!"), Err(MacParseError));
}
