pub fn nth(n: u32) -> u32 {
    let mut sieve = [0; 200_000];

    for i in 2..sieve.len() {
        if sieve[i] != 0 {
            continue;
        }

        let mut j = i * i;
        while j < sieve.len() {
            sieve[j] = 1;
            j += i;
        }
    }

    let mut primes: Vec<u32> = Vec::new();

    for i in 2..sieve.len() {
        if sieve[i] == 0 {
            primes.push(i as u32);
        }
    }

    primes[n as usize]
}
