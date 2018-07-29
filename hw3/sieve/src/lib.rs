pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut size = upper_bound as usize;
    size += 1;
    let mut is_prime = vec![true; size];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..size {
        if is_prime[i] != false {
            let prime_interval = i;
            let mut current = i + prime_interval;
            while current < size {
                is_prime[current] = false;
                current += prime_interval;
            }
        }
    }

    let mut primes = Vec::new();
    for i in 2..size {
        if is_prime[i] {
            primes.push(i as u64);
        }
    }
    primes
}
