pub mod day16_1;
pub mod day16_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day16_2::answer());

    println!("{:?}", start.elapsed());
}
