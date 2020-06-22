use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use threadpool::ThreadPool;

/// Calculates the largest prime number in the provided tuple.
/// The first tuple parameter must be greater than the second tuple parameter.
/// If both of the parameters are equal, the primality of just the provided number
/// is calculated.
///
/// # Examples
///
/// ```
/// use primes;
/// use std::option::Option::Some;
///
/// if let Some(largest_prime) = primes::largest_prime((10, 100)) {
///     assert_eq!(97, largest_prime);
/// }
///```
///
///```
/// if let Some(largest_prime) = primes::largest_prime((11, 11)) {
///     assert_eq!(11, largest_prime);
/// }
///```
///
///```
/// let largest_prime = primes::largest_prime((14, 16));
/// assert!(largest_prime.is_none());
/// ```
///
pub fn largest_prime(args: (usize, usize)) -> Option<usize> {
    let (tx, rx) = mpsc::channel();
    let (start, end) = args;
    let num_cpus = num_cpus::get();
    prime_calculator_job((start, end), tx, num_cpus);
    largest_prime_aggregator((start, end), rx, num_cpus)
}

fn largest_prime_aggregator(
    (start, end): (usize, usize),
    rx: Receiver<Option<usize>>,
    num_cpus: usize,
) -> Option<usize> {
    let handle = thread::spawn(move || {
        let mut count = 0;
        let mut largest_prime = None;
        loop {
            if count < num_cpus {
                if let Ok(v) = rx.try_recv() {
                    count += 1;
                    if v.is_some() {
                        match largest_prime {
                            Some(prime) if v.unwrap() > prime => {
                                largest_prime = v;
                            }
                            None if v.unwrap() > 1 => {
                                largest_prime = v;
                            }
                            _ => (),
                        }
                    }
                }
                continue;
            }
            break;
        }
        largest_prime
    });

    match handle.join() {
        Ok(largest_prime) => {
            match largest_prime {
                Some(largest_prime) => return Some(largest_prime),
                None => {
                    if start == end {
                        println!("{} is not a prime", start);
                    } else {
                        println!("No prime number found between {} and {}", start, end);
                    }
                }
            }
            println!("Finished execution successfully")
        }
        Err(err) => println!("Error : {:#?} occurred", err),
    }
    None
}

fn prime_calculator_job(
    (mut start, mut end): (usize, usize),
    tx: Sender<Option<usize>>,
    num_cpus: usize,
) {
    println!("Number of cpus: {}", num_cpus);

    let job_size = (end - start) / num_cpus;
    let last_job_size = (end - start) - (job_size * (num_cpus - 1));
    println!("Size of each job: {}", job_size);

    let pool = ThreadPool::new(num_cpus);

    let known_primes = if start == 0 { Some(vec![]) } else { None };

    let known_primes = Arc::new(Mutex::new(known_primes));

    for job in 1..=num_cpus {
        let tx = tx.clone();
        if job != 1 {
            start = end;
        }
        if job == num_cpus {
            end = start + last_job_size;
        } else {
            end = start + job_size;
        }

        let known_prime = Arc::clone(&known_primes);

        pool.execute(move || {
            if let Ok(mut lock) = known_prime.lock() {
                if let Ok(mut result) = calculate_prime(start, end, lock.as_ref()) {
                    for r in &result {
                        if let Some(p) = (*lock).as_mut() {
                            p.push(*r);
                        }
                    }

                    if let Some(p) = (*lock).as_mut() {
                        p.sort();
                    } else {
                        result.sort();
                    };

                    let mut last = None;

                    if let Some(p) = lock.as_ref() {
                        last = Some(p[p.len() - 1]);
                    } else if result.len() > 0 {
                        last = Some(result[result.len() - 1]);
                    };

                    tx.send(last).unwrap();
                }
            }
        });
    }
}

fn calculate_prime(
    start: usize,
    end: usize,
    known_primes: Option<&Vec<usize>>,
) -> Result<Vec<usize>, String> {
    let mut primes = vec![];
    for p in start..=end {
        if p == 0 || p == 1 {
            continue;
        }

        let mut is_prime = true;

        if let Some(known_primes) = known_primes {
            for i in known_primes {
                if *i > p {
                    break;
                } else if p % *i == 0 {
                    is_prime = false;
                }
            }
        } else {
            for i in 2..=p / 2 {
                if p % i == 0 {
                    is_prime = false;
                    break;
                }
            }
        }

        if is_prime {
            primes.push(p);
        }
    }
    Ok(primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_prime() {
        let prime = calculate_prime(17, 17, None);

        assert!(prime.is_ok());

        let prime = prime.unwrap();
        assert!(!prime.is_empty());
        assert_eq!(prime, vec![17]);
    }

    #[test]
    fn detects_non_prime() {
        let prime = calculate_prime(217, 217, None);

        assert!(prime.is_ok());
        assert!(prime.unwrap().is_empty());
    }

    #[test]
    fn finds_all_primes_from_0_to_20() {
        let primes = calculate_prime(0, 20, None);

        assert!(primes.is_ok());

        let primes = primes.unwrap();
        assert!(!primes.is_empty());
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn finds_all_primes_from_20_to_40() {
        let primes = calculate_prime(20, 40, None);

        assert!(primes.is_ok());

        let primes = primes.unwrap();
        assert!(!primes.is_empty());
        assert_eq!(primes, vec![23, 29, 31, 37]);
    }

    #[test]
    fn finds_largest_prime_from_20_to_40() {
        let primes = largest_prime((20, 40));

        assert!(primes.is_some());

        let primes = primes.unwrap();
        assert_eq!(primes, 37);
    }

    #[test]
    fn finds_no_largest_prime_from_44_to_46() {
        let primes = largest_prime((44, 46));

        assert!(primes.is_none());
    }
}
