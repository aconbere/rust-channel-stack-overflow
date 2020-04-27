use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, Clone, Copy)]
enum T {
    A,
    // unused, but important in recreating the issue
    // presumably this establishes the size of the value
    // put in the array?
    B
}

fn generate_a_big_array() -> [[T;256];512] {
    [[T::A; 256]; 512]
}

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move|| {
        sender.send(generate_a_big_array()).unwrap();
    });

    let r1 = receiver.recv().unwrap();
    println!("R: {:?}", r1[0][0]);
}
