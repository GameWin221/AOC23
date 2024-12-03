pub mod day14_1;
pub mod day14_1_fast;
pub mod day14_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day14_2::answer());

    println!("{:?}", start.elapsed());
}
