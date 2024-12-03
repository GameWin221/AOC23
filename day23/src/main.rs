pub mod day23_1;
pub mod day23_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day23_1::answer());

    println!("{:?}", start.elapsed());
}
