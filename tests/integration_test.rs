use learn_rust::add_two;

#[test]
fn integration() {

    assert_eq!(4, add_two(2));
}