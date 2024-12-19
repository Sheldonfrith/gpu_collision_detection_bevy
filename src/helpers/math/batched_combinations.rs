use std::collections::HashSet;
use std::iter::FromIterator;

/// Generate all partitions of n with parts limited to max_part
fn partition_k(n: u32, max_part: u32) -> Vec<Vec<u32>> {
    fn partition_recursive(
        n: u32,
        max_part: u32,
        current: &mut Vec<u32>,
        result: &mut Vec<Vec<u32>>,
    ) {
        if n == 0 {
            result.push(current.clone());
            return;
        }

        let limit = std::cmp::min(max_part, n);
        for i in (1..=limit).rev() {
            current.push(i);
            partition_recursive(n - i, std::cmp::min(i, n - i), current, result);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    partition_recursive(n, max_part, &mut current, &mut result);
    result.sort_by(|a, b| b.cmp(a));
    result
}

/// Generate all permutations of a slice
fn permutations<T: Clone + Ord + std::hash::Hash>(items: &[T]) -> HashSet<Vec<T>> {
    if items.is_empty() {
        let mut set = HashSet::new();
        set.insert(vec![]);
        return set;
    }

    let mut result = HashSet::new();
    for i in 0..items.len() {
        let mut smaller = items.to_vec();
        let item = smaller.remove(i);
        for mut perm in permutations(&smaller) {
            perm.push(item.clone());
            result.insert(perm);
        }
    }
    result
}

/// Get elements for a group
fn get_group_elements(group_idx: u32, g: u32, e: u32) -> Vec<u32> {
    let start = group_idx * g;
    let end = std::cmp::min(start + g, e);
    (start + 1..=end).collect()
}

/// Generate combinations of k elements from n elements
fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }

    let mut result = Vec::new();
    for i in 0..=items.len() - k {
        for mut comb in combinations(&items[i + 1..], k - 1) {
            comb.insert(0, items[i].clone());
            result.push(comb);
        }
    }
    result
}

/// Main function to generate memory-constrained combinations
pub fn generate_memory_constrained_combinations(e: u32, k: u32, m: u32) -> Vec<Vec<u32>> {
    let g = m / k; // group size
    let num_groups = (e + g - 1) / g; // ceiling division for number of groups

    // Generate all partitions
    let partitions = partition_k(k, g);

    // Generate permutations for each partition size
    let mut perm_by_parts: Vec<HashSet<Vec<u32>>> = vec![HashSet::new(); (k + 1) as usize];
    for p in partitions.iter() {
        let perms = permutations(p);
        perm_by_parts[p.len()].extend(perms);
    }

    let mut result = HashSet::new();

    // Process each number of parts from k down to 1
    for num_parts in (1..=k as usize).rev() {
        // Generate combinations of group indices
        let group_indices: Vec<u32> = (0..num_groups).collect();
        for group_combo in combinations(&group_indices, num_parts) {
            // Load elements for these groups
            let groups_elements: Vec<Vec<u32>> = group_combo
                .iter()
                .map(|&idx| get_group_elements(idx, g, e))
                .collect();

            // Process each partition with this many parts
            for perm in perm_by_parts[num_parts].iter() {
                // Generate combinations according to partition
                let mut all_combinations = vec![vec![]];

                for (part_idx, &part_size) in perm.iter().enumerate() {
                    let group_elements = &groups_elements[part_idx];
                    let part_combinations = combinations(group_elements, part_size as usize);

                    let mut new_combinations = Vec::new();
                    for existing in all_combinations {
                        for part_comb in part_combinations.iter() {
                            let mut combined = existing.clone();
                            combined.extend(part_comb.iter().cloned());
                            new_combinations.push(combined);
                        }
                    }
                    all_combinations = new_combinations;
                }

                // Add valid combinations to result
                for mut combo in all_combinations {
                    if combo.len() == k as usize {
                        combo.sort_unstable();
                        result.insert(combo);
                    }
                }
            }
        }
    }

    Vec::from_iter(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_constrained_combinations() {
        // Test with the example from the text: e=21, k=3, m=9
        let e = 21;
        let k = 3;
        let m = 9;

        let combs = generate_memory_constrained_combinations(e, k, m);
        println!("Generated {} combinations", combs.len());

        // Calculate the expected number of combinations using the binomial coefficient
        let expected = factorial(e) / (factorial(k) * factorial(e - k));
        assert_eq!(combs.len() as u64, expected);

        // Verify all combinations are unique
        let unique_combs: HashSet<Vec<u32>> = HashSet::from_iter(combs.clone());
        assert_eq!(unique_combs.len(), combs.len());

        // Verify each combination has exactly k elements
        for combo in combs {
            assert_eq!(combo.len(), k as usize);
        }
    }

    fn factorial(n: u32) -> u64 {
        (1..=n as u64).product()
    }
}
// [Previous code remains the same until the test module]
