pub mod day18_1;
pub mod day18_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day18_2::answer());

    println!("{:?}", start.elapsed());
}
