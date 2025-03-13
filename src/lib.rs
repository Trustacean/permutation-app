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
