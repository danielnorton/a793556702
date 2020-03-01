use a793556702::say_hello;

#[test]
fn do_it() {

    let message = say_hello("Ted");
    assert_eq!("Hello, Ted!", message);
}