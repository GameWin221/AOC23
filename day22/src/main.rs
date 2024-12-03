pub mod day22_1;
//pub mod day22_2;

fn main() {
    let start = std::time::Instant::now();

    println!("{}", day22_1::answer());

    println!("{:?}", start.elapsed());
}
