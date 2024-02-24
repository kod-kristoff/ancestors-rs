use super::*;

#[test]
fn person_id_is_default() {
    let pid = PersonId::default();
    assert!(pid.as_str().starts_with("p#"));
}

#[test]
fn try_from_string_succeeds() {
    let pid1 = PersonId::default();
    let pid2 = PersonId::try_from(pid1.clone().to_string()).unwrap();
    assert_eq!(pid1, pid2);
}

#[test]
fn try_from_string_fails() {
    let pid = PersonId::try_from("++ ++".to_string());
    assert!(pid.is_err());
}
