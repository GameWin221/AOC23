pub mod day10_1;
pub mod day10_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day10_2::answer());

    println!("{:?}", start.elapsed());
}