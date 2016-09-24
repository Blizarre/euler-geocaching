// https://projecteuler.net/problem=14

#[cfg(test)]
mod test_solver {
    use super::Solver;

    #[test]
    fn test_nb_hops() {
        let mut solver = Solver::new(1000);
        assert_eq!(solver.number_of_hops(6), 9);
        assert_eq!(solver.number_of_hops(97), 119);
        assert_eq!(solver.number_of_hops(6), 9);
        assert_eq!(solver.number_of_hops(97), 119);
        assert_eq!(solver.number_of_hops(12), 10);
        assert_eq!(solver.number_of_hops(24), 11);
        assert_eq!(solver.number_of_hops(97 * 2), 119 + 1);

        let mut solver2 = Solver::new(1000);
        assert_eq!(solver2.number_of_hops(24), 11);
        assert_eq!(solver2.number_of_hops(6), 9);
    }

}

#[derive(Debug, Clone)]
struct Solver {
    // Vector with index = number, value = Result(Number of hops for number)
    nb_hops_cache: Vec<Option<u64>>,
}

impl Solver {
    fn new(max_number: usize) -> Solver {
        Solver { nb_hops_cache: vec![None; max_number] }
    }

    fn is_in_cache(&self, nb: u64) -> bool {
        nb < self.nb_hops_cache.len() as u64 && self.nb_hops_cache[nb as usize].is_some()
    }

    fn number_of_hops(&mut self, nb: u64) -> u64 {
        if self.is_in_cache(nb) {
            return self.nb_hops_cache[nb as usize].unwrap();
        }

        // list of intermediate numbers found when computing number_of_hops(n)
        let mut hops = Vec::<u64>::with_capacity(100);

        let mut new_nb = nb;
        while new_nb != 1 && !self.is_in_cache(new_nb) {
            hops.push(new_nb);
            new_nb = match new_nb % 2 {
                0 => new_nb / 2,
                1 => new_nb * 3 + 1,
                _ => panic!("Math error"),
            };
        }
        // found the end, fill the nb_hops_cache array, we know new_nb is either in the cache index or 1
        let mut hops_index:u64 = match self.nb_hops_cache[new_nb as usize] {
            // Found new_nb in cache: just return the number of hops in cache for new_nb
            Some(x) => x,
            // Didn't find it, new_nb is 1, return the index for 1 (first element)
            None => 1,
        };
        // going backward, fill the cache and increment the hops_index counter each time
        for i in hops.iter().rev() {
            hops_index += 1;
            // cache the items if they fit in the cache
            if *i < (usize::max_value() as u64) && (*i as usize) < self.nb_hops_cache.len() {
                self.nb_hops_cache[*i as usize] = Some(hops_index);
            }
        }
        hops_index
    }
}

// we need to get index of each element of the maximum length chain for geocaching
fn print_geo_result(mut nb: u64) {
    let mut index: usize = 1;
    println!("[{}] = {}", index, nb);
    while nb != 1 {
        index += 1;
        nb = match nb % 2 {
            0 => nb / 2,
            1 => nb * 3 + 1,
            _ => panic!("Math error"),
        };
        println!("[{}] = {}", index, nb);
    }

}

fn main() {
    let nb_max: u64 = 11000000;
    let mut max_nb_hops = 0;
    let mut max_index = 0;
    let mut nb_hops: u64;

    let mut solver = Solver::new(nb_max as usize);
    // todo: use max and an iterator
    // nb: index goes from 1 to MAX
    for i in 1..nb_max {
        nb_hops = solver.number_of_hops(i);
        if nb_hops > max_nb_hops {
            max_nb_hops = nb_hops;
            max_index = i
        }
    }
    println!("Maximum number of hops is {} for number {}",
             max_nb_hops,
             max_index);
    print_geo_result(max_index);
}
