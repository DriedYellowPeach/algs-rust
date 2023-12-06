use crate::queue;

pub fn my_stack() {
    queue::my_queue();
    println!("this is my stack");
}

#[test]
fn test_my_stack() {
    my_stack();
}
