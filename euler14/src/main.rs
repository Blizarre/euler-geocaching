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

        let mut solver2 = Solver::new(1000);
        assert_eq!(solver2.number_of_hops(24), 11);
        assert_eq!(solver2.number_of_hops(6), 9);
    }

}

#[derive(Debug, Clone)]
struct Solver {
    data: Vec<Option<usize>>,
}

impl Solver {
    fn new(max_number: usize) -> Solver {
        Solver { data: vec![None; max_number] }
    }

    fn is_in_cache(&self, nb: usize) -> bool {
        nb < self.data.len() && self.data[nb].is_some()
    }

    fn number_of_hops(&mut self, nb: usize) -> usize {
        if self.data[nb].is_some() {
            return self.data[nb].unwrap();
        }

        let mut hops = Vec::<usize>::with_capacity(100);

        let mut new_nb = nb;
        while new_nb != 1 && !self.is_in_cache(new_nb) {
            hops.push(new_nb);
            new_nb = match new_nb % 2 {
                0 => new_nb / 2,
                1 => new_nb * 3 + 1,
                _ => panic!("Math error"),
            };
        }
        // found the end, fill the data array
        let mut hops_nb = match self.data[new_nb] {
            Some(x) => x as usize,
            None => 1,
        };
        for i in hops.iter().rev() {
            hops_nb += 1;
            // println!("Adding {} for hops {}", *i, hops_nb);
            if *i < self.data.len() {
                self.data[*i] = Some(hops_nb);
            }
        }
        hops_nb
    }
}

fn print_geo_result(mut nb: usize) {
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
    let nb_max: usize = 10000000;
    let mut max_nb_hops = 0;
    let mut max_index = 0;
    let mut nb_hops: usize;

    let mut solver = Solver::new(nb_max);
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
