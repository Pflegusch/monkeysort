use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::process;
use std::time::Instant;

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn sort<'a>(vec: &'a mut Vec<u64>, shuffles: &mut u128) -> &'a mut Vec<u64> {
    let mut sorted: bool = false;

    while !sorted {
        vec.shuffle(&mut thread_rng());
        *shuffles += 1;
        sorted = is_sorted(vec);
    }

    vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./monkeysort <upper> <print: true/false>");
        process::exit(-1);
    }

    let upper: usize = args[1].parse::<usize>().unwrap();
    let debug: bool = args[2].parse::<bool>().unwrap();

    let range = Uniform::from(0..u64::MAX);
    let sizes: Vec<usize> = (2..upper).collect();
    let mut vec: Vec<u64>;

    for size in sizes.iter() {
        vec = rand::thread_rng().sample_iter(&range).take(*size).collect();
        let mut shuffles = 0;
        let start = Instant::now();
        sort(&mut vec, &mut shuffles);
        if debug {
            for num in vec.iter() {
                println!("{}", num);
            }
        }
        let duration = start.elapsed();
        println!(
            "Running with {} elements took: {:?} and {} shuffles",
            size, duration, shuffles
        );
    }
}
