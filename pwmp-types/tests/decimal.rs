use pwmp_types::decimal::Decimal;

#[test]
fn decimal_to_string() {
    assert_eq!(Decimal::new(4, 20).to_string(), "4.20".to_string());
    assert_eq!(Decimal::new(-3, 7).to_string(), "-3.7".to_string());
    assert_eq!(Decimal::new(64, 92).to_string(), "64.92".to_string());
    assert_eq!(Decimal::new(-64, 92).to_string(), "-64.92".to_string());
}

#[test]
fn decimal_from_f32() {
    assert_eq!(Decimal::from_f32(4.20, 2), Decimal::new(4, 20));
    assert_eq!(Decimal::from_f32(-3.7, 2), Decimal::new(-3, 70));
    assert_eq!(Decimal::from_f32(64.92, 2), Decimal::new(64, 92));
    assert_eq!(Decimal::from_f32(-64.92, 2), Decimal::new(-64, 92));
}

#[test]
fn decimal_cmp() {
    assert_eq!(Decimal::new(3, 5), Decimal::new(3, 5));
    assert_eq!(Decimal::new(-45, 2), Decimal::new(-45, 2));
}

#[test]
fn decimal_count_scale() {
    assert_eq!(Decimal::new(4, 20).scale(), 2);
    assert_eq!(Decimal::new(-3, 7).scale(), 1);
    assert_eq!(Decimal::new(64, 92).scale(), 2);
    assert_eq!(Decimal::new(-64, 92).scale(), 2);
    assert_eq!(Decimal::new(23, 112).scale(), 3);
}
