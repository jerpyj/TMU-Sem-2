#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Add your functions for lab 8 below. Fuction skeletons with dummy return values 
    are provided, edit them as needed. You may also add additional helper functions. 
    
    Test your code by running 'cargo test' from the lab8 directory.
*/

use std::fmt;
use std::ops;
use std::cmp;

struct Poly {
    coeffs: Vec<i32>,
}

impl ops::Add<Poly> for Poly {
    type Output = Poly;
    fn add(self, other: Poly) -> Poly {
        //compare 2 ints and get max for length
        let max = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = Vec::new();

        for i in 0..max {
            let x = self.coeffs.get(i).unwrap_or(&0);
            let y = other.coeffs.get(i).unwrap_or(&0);
            result.push(x + y);
        }
        Poly{coeffs: result}
    }
}
    
impl ops::Sub<Poly> for Poly {
    type Output = Poly;
    fn sub(self, other: Poly) -> Poly {
        //compare 2 ints and get max for length
        let max = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = Vec::new();

        for i in 0..max {
            let x = self.coeffs.get(i).unwrap_or(&0);
            let y = other.coeffs.get(i).unwrap_or(&0);
            result.push(x - y);
        }
        Poly{coeffs: result}
    }
}
    
impl cmp::PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        let max = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = true;

        for i in 0..max {
            let x = self.coeffs.get(i).unwrap_or(&0);
            let y = other.coeffs.get(i).unwrap_or(&0);
            if x != y {
                result = false;
            }
        }
        result
    }
}
    
impl ops::Mul<Poly> for Poly {
    type Output = Poly;
    fn mul(self, other: Poly) -> Poly {
        let lenSelf = self.coeffs.len();
        let lenOther = other.coeffs.len();

        let mut result = vec![0; lenSelf + lenOther - 1];

        for i in 0..lenSelf{
            for j in 0..lenOther{
                let x = self.coeffs[i];
                let y = other.coeffs[j];

                result [i + j] += x * y;
            }
        }

        Poly{coeffs: result}
    }
}
    
impl ops::Mul<i32> for Poly {
    type Output = Poly;
    fn mul(self, other: i32) -> Poly {
        let mut result = Vec::new();

        for i in 0..self.coeffs.len() {
            let x = self.coeffs[i];
            result.push(x * other);
        }
        Poly{coeffs: result}
    }
}
    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
