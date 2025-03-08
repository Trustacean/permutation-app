use std::collections::HashSet;
use std::fmt;

fn main() {
    let nrof_males = 3;
    let nrof_felames = 2;
    let nrof_chairs = 5;

    let population = create_population(nrof_males, nrof_felames);

    let mut permutations: Vec<Vec<String>> = Vec::new();
    let mut permutation = Vec::new();
    let mut used = HashSet::new();

    free_permute(
        &population,
        nrof_chairs,
        &mut permutation,
        &mut used,
        &mut permutations,
    );

    for (i, permutation) in permutations.iter().enumerate() {
        println!("Permutation {}: {:?}", i + 1, permutation);
    }
}

fn create_population(nrof_males: i32, nrof_females: i32) -> Vec<String> {
    let mut population: Vec<String> = Vec::new();

    for i in 0..nrof_males {
        population.push(format!("M{}", i + 1));
    }

    for i in 0..nrof_females {
        population.push(format!("F{}", i + 1));
    }

    population
}

fn free_permute(
    population: &[String],
    nrof_chairs: usize,
    permutation: &mut Vec<String>,
    used: &mut HashSet<usize>,
    permutations: &mut Vec<Vec<String>>,
) {
    // If the permutation is already full, add it to the list of permutations
    if permutation.len() == nrof_chairs {
        permutations.push(permutation.clone());
        return;
    }

    for (i, person) in population.iter().enumerate() {
        if !used.contains(&i) {
            // Mark the person as used
            used.insert(i);
            permutation.push(person.clone());

            // Recurse
            free_permute(population, nrof_chairs, permutation, used, permutations);

            // Backtrack
            permutation.pop();
            used.remove(&i);
        }
    }
}

enum PermutationError {
    PopulationLargerThanChairs,
}

impl fmt::Debug for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermutationError::PopulationLargerThanChairs => {
                write!(f, "Population is larger than the number of chairs")
            }
        }
    }
}
