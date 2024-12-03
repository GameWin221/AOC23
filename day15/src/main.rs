pub mod day15_1;
pub mod day15_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day15_2::answer());

    println!("{:?}", start.elapsed());
}
