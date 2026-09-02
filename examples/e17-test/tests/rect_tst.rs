use tester;

#[test]
fn integration_test_1() {
    let r = tester::add(1, 5);
    assert_eq!(6, r);
}