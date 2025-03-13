use permutation_app::*;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let nrof_males = 5;
    let nrof_females = 4;

    let population = create_population(nrof_males, nrof_females);

    let permutations = free_permutations(&population);

    let elapsed_time = start_time.elapsed();

    for (i, perm) in permutations.iter().enumerate() {
        println!("Permutation {}: {:?}", i + 1, perm);
    }

    println!("Elapsed time (Without Print): {:?}", elapsed_time);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time (With Print): {:?}", elapsed_time);
}
