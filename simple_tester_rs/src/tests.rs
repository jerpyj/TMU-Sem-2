#[cfg(test)]
mod tests {
    
    use super::super::{*};
    use std::fs::File;
    use std::io::Write;
    use ntest::timeout;
    use std::time::Instant;
    use rand::{Rng, SeedableRng, };
    use rand::rngs::StdRng;
    use rand::prelude::*;

    const SEED: u64 = 13;

    #[test]
    #[timeout(60000)]
    fn test_1_small() -> std::io::Result<()>
    {
        const NLINES: usize = 5;

        let (p1, p2, linevec) = get_inputs(NLINES);
        let mut lines: [u32; NLINES] = [0; NLINES];
        for i in 0..NLINES { lines[i] = linevec[i]; }
        
        let start = Instant::now();
        let sol = segment(&p1, &p2, &lines);
        let stop = start.elapsed().as_millis();
        let result = verify_sol(&p1, &p2, &lines, &sol);
        
        let mut file = File::create("test_1_small.txt")?;
        let mut toprint = String::from("SMALL - 5 LINES:\n");
        toprint.push_str(format!("  Test 1: {:?}, {:?}, {:?}\n", p1, p2, lines).as_str());
        toprint.push_str(format!("    You returned \"{}\" in {}ms\n", sol, stop).as_str());
        toprint.push_str(format!("    {}\n\n", result).as_str());
        file.write_all(toprint.as_bytes())?;

        assert!(result == "PASSED!");
        Ok(())
    }

    #[test]
    #[timeout(60000)]
    fn test_2_medium() -> std::io::Result<()>
    {
        const NLINES: usize = 10;

        let (p1, p2, linevec) = get_inputs(NLINES);
        let mut lines: [u32; NLINES] = [0; NLINES];
        for i in 0..NLINES { lines[i] = linevec[i]; }
        
        let start = Instant::now();
        let sol = segment(&p1, &p2, &lines);
        let stop = start.elapsed().as_millis();
        let result = verify_sol(&p1, &p2, &lines, &sol);
        
        let mut file = File::create("test_2_medium.txt")?;
        let mut toprint = String::from("MEDIUM - 10 LINES:\n");
        toprint.push_str(format!("  Test 1: {:?}, {:?}, {:?}\n", p1, p2, lines).as_str());
        toprint.push_str(format!("    You returned \"{}\" in {}ms\n", sol, stop).as_str());
        toprint.push_str(format!("    {}\n\n", result).as_str());
        file.write_all(toprint.as_bytes())?;

        assert!(result == "PASSED!");
        Ok(())
    }

    #[test]
    #[timeout(60000)]
    fn test_3_large() -> std::io::Result<()>
    {
        const NLINES: usize = 20;

        let (p1, p2, linevec) = get_inputs(NLINES);
        let mut lines: [u32; NLINES] = [0; NLINES];
        for i in 0..NLINES { lines[i] = linevec[i]; }
        
        let start = Instant::now();
        let sol = segment(&p1, &p2, &lines);
        let stop = start.elapsed().as_millis();
        let result = verify_sol(&p1, &p2, &lines, &sol);
        
        let mut file = File::create("test_3_large.txt")?;
        let mut toprint = String::from("LARGE - 20 LINES:\n");
        toprint.push_str(format!("  Test 1: {:?}, {:?}, {:?}\n", p1, p2, lines).as_str());
        toprint.push_str(format!("    You returned \"{}\" in {}ms\n", sol, stop).as_str());
        toprint.push_str(format!("    {}\n\n", result).as_str());
        file.write_all(toprint.as_bytes())?;

        assert!(result == "PASSED!");
        Ok(())
    }

    #[test]
    #[timeout(60000)]
    fn test_4_mega() -> std::io::Result<()>
    {
        const NLINES: usize = 50;

        let (p1, p2, linevec) = get_inputs(NLINES);
        let mut lines: [u32; NLINES] = [0; NLINES];
        for i in 0..NLINES { lines[i] = linevec[i]; }
        
        let start = Instant::now();
        let sol = segment(&p1, &p2, &lines);
        let stop = start.elapsed().as_millis();
        let result = verify_sol(&p1, &p2, &lines, &sol);
        
        let mut file = File::create("test_4_mega.txt")?;
        let mut toprint = String::from("MEGA - 50 LINES:\n");
        toprint.push_str(format!("  Test 1: {:?}, {:?}, {:?}\n", p1, p2, lines).as_str());
        toprint.push_str(format!("    You returned \"{}\" in {}ms\n", sol, stop).as_str());
        toprint.push_str(format!("    {}\n\n", result).as_str());
        file.write_all(toprint.as_bytes())?;

        assert!(result == "PASSED!");
        Ok(())
    }

    #[test]
    #[timeout(60000)]
    fn test_5_giga() -> std::io::Result<()>
    {
        const NLINES: usize = 100;

        let (p1, p2, linevec) = get_inputs(NLINES);
        let mut lines: [u32; NLINES] = [0; NLINES];
        for i in 0..NLINES { lines[i] = linevec[i]; }
        
        let start = Instant::now();
        let sol = segment(&p1, &p2, &lines);
        let stop = start.elapsed().as_millis();
        let result = verify_sol(&p1, &p2, &lines, &sol);
        
        let mut file = File::create("test_5_giga.txt")?;
        let mut toprint = String::from("GIGA - 100 LINES:\n");
        toprint.push_str(format!("  Test 1: {:?}, {:?}, {:?}\n", p1, p2, lines).as_str());
        toprint.push_str(format!("    You returned \"{}\" in {}ms\n", sol, stop).as_str());
        toprint.push_str(format!("    {}\n\n", result).as_str());
        file.write_all(toprint.as_bytes())?;

        assert!(result == "PASSED!");
        Ok(())
    }

    fn get_inputs(n: usize) -> ((i32,i32), (i32,i32), Vec<u32>)
    {
        let mut rng = StdRng::seed_from_u64(SEED);
        let mut lines: Vec<u32> = Vec::new();
        let mut dirs: Vec<char> = Vec::new();

        for _ in 0..n {
            if let Some(random_char) = "UDLR".chars().choose(&mut rng) {
                dirs.push(random_char);
            }
            else { }
            lines.push(rng.gen_range(1..=10));
        }

        let x = 10 - rng.gen_range(0..=20) as i32; 
        let y = 10 - rng.gen_range(0..=20) as i32;
        let mut dx = x;
        let mut dy = y;

        for i in 0..n {
            match dirs[i] {
                'U' => { dy += lines[i] as i32 }
                'D' => { dy -= lines[i] as i32 }
                'L' => { dx -= lines[i] as i32 }
                'R' => { dx += lines[i] as i32 }
                 _  => { }
            }
        }

        ((x, y), (dx, dy), lines)
    }

    fn verify_sol(p1: &(i32,i32), p2: &(i32,i32), lines: &[u32], sol: &String) -> String
    {
        if sol.len() != lines.len() {
            return String::from("FAILED: Solution length must match number of lines.");
        }
        
        for c in sol.chars() {
            if c != 'U' && c != 'D' && c != 'L' && c != 'R' {
                return String::from("FAILED: Solution should only contain U, D, R, L.");
            }
        }

        let mut dx = p1.0;
        let mut dy = p1.1;
        for i in 0..lines.len() {
            match sol.chars().nth(i) {
                Some('U') => { dy += lines[i] as i32 }
                Some('D') => { dy -= lines[i] as i32 }
                Some('L') => { dx -= lines[i] as i32 }
                Some('R') => { dx += lines[i] as i32 }
                 _  => { }
            }
        }
        if dx != p2.0 || dy != p2.1 {
            return String::from("FAILED: Finish point incorrect.");
        }

        String::from("PASSED!")
    }

}

