use super::{infimum_of, roar, Infimum};

#[test]
fn binary_search() {
    assert!(matches!(infimum_of(0), Infimum::Eqaul));
    assert!(matches!(infimum_of(2025), Infimum::Less(2024)));
    assert!(matches!(infimum_of(114514), Infimum::Eqaul));
    assert!(matches!(infimum_of(229028), Infimum::Eqaul));
    assert!(matches!(infimum_of(300000), Infimum::Less(229028)));
}

#[test]
#[ignore]
fn bigint() {
    let num = "99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               99999999999999999999999999\
               9999999999999999999999";

    println!("{}", roar(num).unwrap());
}

#[test]
fn positive() {
    let formula: &str = &roar("145").unwrap();

    assert_eq!(formula, "1+145+1-4-11+4-5+14");
}

#[test]
fn minus() {
    let formula: &str = &roar("-145").unwrap();

    assert_eq!(formula, "(11-4-5+1-4)*(1+145+1-4+(-11+4-5+14))");
}

#[test]
fn zero() {
    assert_eq!(roar("0"), roar("-0"));
}

#[test]
fn invalid_chars() {
    assert!(roar("悲").is_err());
    assert!(roar("--145").is_err());
}
