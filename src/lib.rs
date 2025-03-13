use std::{
    fmt,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone)]
pub enum Individual {
    Male(u32),
    Female(u32),
}

impl fmt::Debug for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Male(i) => write!(f, "M{}", i),
            Self::Female(i) => write!(f, "F{}", i),
        }
    }
}

pub fn create_population(nrof_males: i32, nrof_females: i32) -> Vec<Individual> {
    let mut population: Vec<Individual> = Vec::new();

    for i in 0..nrof_males {
        population.push(Individual::Male((i as u32) + 1));
    }

    for i in 0..nrof_females {
        population.push(Individual::Female((i as u32) + 1));
    }

    population
}

pub fn free_permutations(population: &[Individual]) -> Vec<Vec<&Individual>> {
    let pop_size = population.len();
    let permutations = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Divide the work into chunks (e.g., one chunk per thread)
    let num_threads = 16; // Adjust based on your CPU cores
    let chunk_size = pop_size / num_threads;

    for thread_id in 0..num_threads {
        let permutations = Arc::clone(&permutations);

        let handle = thread::spawn(move || {
            let mut local_permutations = Vec::new();
            let start = thread_id * chunk_size;
            let end = if thread_id == num_threads - 1 {
                pop_size
            } else {
                (thread_id + 1) * chunk_size
            };

            for i in start..end {
                let mut permutation = Vec::new();
                let used = 1 << i;
                permutation.push(i);
                free_permute(pop_size, &mut permutation, used, &mut local_permutations);
            }

            let mut global_permutations = permutations.lock().unwrap();
            global_permutations.extend(local_permutations);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Convert indices to references to individuals
    let permutations = Arc::try_unwrap(permutations).unwrap().into_inner().unwrap();
    permutations
        .into_iter()
        .map(|perm_indices| perm_indices.iter().map(|&idx| &population[idx]).collect())
        .collect()
}

pub fn free_permute(
    pop_size: usize,
    permutation: &mut Vec<usize>,
    used: u32,
    permutations: &mut Vec<Vec<usize>>,
) {
    // If the permutation is already full, add it to the list of permutations
    if permutation.len() == pop_size {
        permutations.push(permutation.clone());
        return;
    }

    for i in 0..pop_size {
        if (used & (1 << i)) == 0 {
            // Mark the person as used
            permutation.push(i);
            free_permute(pop_size, permutation, used | (1 << i), permutations);
            // Backtrack
            permutation.pop();
        }
    }
}
