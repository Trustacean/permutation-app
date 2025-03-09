use std::collections::HashSet;

pub fn create_population(nrof_males: i32, nrof_females: i32) -> Vec<String> {
    let mut population: Vec<String> = Vec::new();

    for i in 0..nrof_males {
        population.push(format!("M{}", i + 1));
    }

    for i in 0..nrof_females {
        population.push(format!("F{}", i + 1));
    }

    population
}

pub fn free_permute(
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