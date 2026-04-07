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
        // Your + code here
        Poly{coeffs: Vec::new()}
    }
}
    
impl ops::Sub<Poly> for Poly {
    type Output = Poly;
    fn sub(self, other: Poly) -> Poly {
        // Your - code here	
        Poly{coeffs: Vec::new()}
    }
}
    
impl cmp::PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        // Your == code here
        false
    }
}
    
impl ops::Mul<Poly> for Poly {
    type Output = Poly;
    fn mul(self, other: Poly) -> Poly {
        // Your * code here
        Poly{coeffs: Vec::new()}
    }
}
    
impl ops::Mul<i32> for Poly {
    type Output = Poly;
    fn mul(self, other: i32) -> Poly {
        // Your * code here
        Poly{coeffs: Vec::new()}
    }
}
    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
