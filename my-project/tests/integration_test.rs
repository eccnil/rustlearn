use my_project::adder_two;


#[test]
fn integration_test(){
    assert_eq!(4, adder_two(2));
}
