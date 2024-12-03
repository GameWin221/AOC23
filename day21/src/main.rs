pub mod day21_1;
pub mod day21_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day21_2::answer());

    println!("{:?}", start.elapsed());
}