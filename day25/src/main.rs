pub mod day25_1;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day25_1::answer());

    println!("{:?}", start.elapsed());
}
