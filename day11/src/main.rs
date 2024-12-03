pub mod day11_1;
pub mod day11_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day11_2::answer());

    println!("{:?}", start.elapsed());
}