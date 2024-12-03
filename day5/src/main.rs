pub mod day5_1;
pub mod day5_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day5_2::answer());

    println!("{:?}", start.elapsed());
}