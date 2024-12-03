pub mod day12_1;
pub mod day12_1_fast;
pub mod day12_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day12_2::answer());

    println!("{:?}", start.elapsed());
}
