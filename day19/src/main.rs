pub mod day19_1;
pub mod day19_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day19_2::answer());

    println!("{:?}", start.elapsed());
}
