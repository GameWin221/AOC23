pub mod day20_1;
pub mod day20_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day20_2::answer());

    println!("{:?}", start.elapsed());
}