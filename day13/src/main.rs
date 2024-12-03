pub mod day13_1;
pub mod day13_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day13_2::answer());
    
    println!("{:?}", start.elapsed());
}