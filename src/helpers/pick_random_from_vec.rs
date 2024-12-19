use rand::Rng;

pub fn pick_random_from_vec<T: Clone>(vec: &Vec<T>, num: usize) -> Vec<T> {
    if vec.is_empty() {
        return vec![];
    }
    let mut rng = rand::thread_rng();
    let mut picked = vec![];
    for _ in 0..num {
        let index = rng.gen_range(0..vec.len());
        picked.push(vec.get(index).unwrap().clone());
    }
    picked
}

//  implement tests for MyRads
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_pick_random() {
        // ensure original vec is not mutated
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let vec_ref = vec.clone();
        let picked = pick_random_from_vec(&vec, 3);
        let picked2 = pick_random_from_vec(&vec, 3);
        assert_ne!(picked, picked2);
        assert_eq!(vec, vec_ref);
        let vec3: Vec<i32> = vec![];
        let picked = pick_random_from_vec(&vec3, 3);
        assert_eq!(picked.len(), 0);
    }
}
