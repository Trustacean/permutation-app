use permutation_app::*;
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
