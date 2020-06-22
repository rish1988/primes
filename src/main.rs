use std::env::Args;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{env, thread};

use num_cpus;
use threadpool::ThreadPool;

fn main() {
    let mut args = env::args();
    let parsed_args = parse_args(&mut args);
    if let Ok(args) = parsed_args {
        largest_prime(args)
    } else if let Err(err) = parsed_args {
        eprintln!("{}", err.to_string());
    }
}

fn largest_prime(args: (usize, usize)) {
    let (mut start, mut end) = args;
    let cpu_count = num_cpus::get();
    println!("Number of cpus: {}", cpu_count);

    let job_size = (end - start) / cpu_count;
    let last_job_size = (end - start) - (job_size * (cpu_count - 1));
    println!("Size of each job: {}", job_size);

    let pool = ThreadPool::new(cpu_count);

    let known_primes = if start == 0 { Some(vec![]) } else { None };

    let known_primes = Arc::new(Mutex::new(known_primes));
    let (tx, rx) = mpsc::channel();
    for job in 1..=cpu_count {
        let tx = tx.clone();
        if job != 1 {
            start = end;
        }
        if job == cpu_count {
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

                    let mut last = 0;

                    if let Some(p) = lock.as_ref() {
                        last = p[p.len() - 1];
                    } else if result.len() > 0 {
                        last = result[result.len() - 1];
                    };

                    tx.send(last).unwrap();
                }
            }
        });
    }

    let known_prime = Arc::clone(&known_primes);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(10));
        println!(
            "Total known primes: {}",
            known_prime.lock().unwrap().as_ref().unwrap().len()
        );
    });

    let handle = thread::spawn(move || {
        let mut count = 0;
        let mut largest_prime = None;
        loop {
            if count < cpu_count {
                match rx.try_recv() {
                    Ok(v) => {
                        count += 1;
                        match largest_prime {
                            Some(prime) if v > prime => {
                                largest_prime = Some(v);
                            }
                            None if v > 1 => {
                                largest_prime = Some(v);
                            }
                            _ => (),
                        }
                    }
                    _ => (),
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
                Some(largest_prime) => println!("Largest prime: {}", largest_prime),
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
        Err(err) => {
            println!("Error : {:#?} occurred", err)
        }
    }
}

fn parse_args(args: &mut Args) -> Result<(usize, usize), String> {
    args.next();

    let (mut start, mut end): (usize, usize) = (0, 0);

    if let Some(s) = args.next() {
        start = s.parse::<usize>().unwrap();
    }

    let end_option = args.next();
    if let Some(e) = end_option {
        end = e.parse::<usize>().unwrap()
    } else if let None = end_option {
        end = start;
    }

    if start > end {
        return Err(String::from("Start is greater than end"));
    }

    Ok((start, end))
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
