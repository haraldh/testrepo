fn main() {
    use std::{thread, time};

    let ten_secs = time::Duration::from_secs(10);
    let now = time::Instant::now();

    thread::sleep(ten_secs);

    assert!(now.elapsed() >= ten_secs);

    println!("Hello, worlds!");
}
