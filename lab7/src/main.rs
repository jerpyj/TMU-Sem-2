#![allow(non_snake_case, non_camel_case_types, dead_code)]

/*
    Add your functions for lab 7 below. Fuction skeletons with dummy return values
    are provided, edit them as needed. You may also add additional helper functions.

    Test your code by running 'cargo test' from the lab7 directory.
*/

fn count_peaks(items: &[i32]) -> u32 {
    let len = items.len();
    if len == 0 {
        return 0;
    }
    if len == 1 {
        return 1;
    }

    let mut peakTotal = 0;

    for i in 0..len {
        let peak = if i == 0 {
            items[0] > items[1]
        } else if i == len - 1 {
            items[i] > items[i - 1]
        } else {
            items[i] > items[i - 1] && items[i] > items[i + 1]
        };

        if peak {
            peakTotal += 1;
        }
    }

    peakTotal
}

fn remove_runs(items: &[i32]) -> Vec<i64> {
    let mut vecky = vec![];

    let len = items.len();
    if len == 0 {
        return vecky;
    }

    vecky.push(items[0] as i64);

    for i in 1..len {
        if items[i] != items[i - 1] {
            vecky.push(items[i] as i64)
        }
    }

    vecky
}

fn count_and_remove_primes(items: &mut [u32]) -> u32 {
    let mut count = 0;

    for i in items.iter_mut() {
        if check_if_prime(*i) {
            *i = 0;
            count += 1;
        }
    }
    count
}

fn safe_squares_rooks(n: u8, rooks: &[(u8, u8)]) -> u32 {
    let size = n as usize;
    if size == 0 {
        return 0;
    }

    //create a vector and set all of it to false
    let mut row = vec![false; size];
    let mut col = vec![false; size];

    // iterate through rooks and find them
    for &(r, c) in rooks {
        row[r as usize] = true;
        col[c as usize] = true;
    }

    //check how many valid rows
    let mut validRow = 0;
    for i in 0..size {
        if row[i] == false {validRow += 1;}
    }

    //check how many valid cols
    let mut validCol = 0;
    for i in 0..size {
        if col[i] == false {validCol += 1;}
    }

    //calculate on board

    (validRow as u32 ) * (validCol as u32)

}

//Dedicated function to check prime numbers
fn check_if_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
