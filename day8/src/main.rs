pub mod day8_1;
pub mod day8_2_bruteforce;
pub mod day8_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day8_2::answer());

    println!("{:?}", start.elapsed());
}