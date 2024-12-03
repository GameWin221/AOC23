pub mod day9_1;
pub mod day9_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day9_2::answer());

    println!("{:?}", start.elapsed());
}