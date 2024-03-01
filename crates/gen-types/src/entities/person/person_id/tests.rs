use super::*;

#[test]
fn person_id_is_default() {
    let pid = PersonId::default();
    assert!(pid.to_string().starts_with("Pers_"));
}

#[test]
fn try_from_string_succeeds() {
    let pid1 = PersonId::default();
    let pid2 = pid1.to_string().parse::<PersonId>().unwrap();
    assert_eq!(pid1, pid2);
}

#[test]
fn try_from_string_fails() {
    let pid = "++ ++".parse::<PersonId>();
    assert!(pid.is_err());
}
