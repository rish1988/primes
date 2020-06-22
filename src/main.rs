use std::env::{self, Args};

fn main() {
    let mut args = env::args();
    let parsed_args = parse_args(&mut args);
    if let Ok(args) = parsed_args {
        if let Some(p) = primes::largest_prime(args) {
            println!("Largest prime found is {}", p);
        }
    } else if let Err(err) = parsed_args {
        eprintln!("{}", err.to_string());
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
