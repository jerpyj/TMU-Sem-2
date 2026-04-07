#[cfg(test)]
mod tests {
    // we adjust the path to:
    use super::super::{*};

    #[test]
    fn poly_add_test() {
        let p1 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let p2 = Poly { coeffs: vec![2, 4, 1], };
        let p3 = p1 + p2;
        assert_eq!(p3.coeffs, vec![5, 6, 3, 1, 4]);
    }

    #[test]
    fn poly_sub_test() {
        let p1 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let p2 = Poly { coeffs: vec![2, 4, 1], };
        let p3 = p1 - p2;
        assert_eq!(p3.coeffs, vec![1, -2, 1, 1, 4]);
    }

    #[test]
    fn poly_mul1_test() {
        let p1 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let p2 = Poly { coeffs: vec![2, 4, 1], };
        let p3 = p1 * p2;
        assert_eq!(p3.coeffs, vec![6, 16, 15, 12, 14, 17, 4]);
    }

    #[test]
    fn poly_mul2_test() {
        let p1 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let p3 = p1 * 2;
        assert_eq!(p3.coeffs, vec![6, 4, 4, 2, 8]);
    }
    
    #[test]
    fn poly_eq_test() {
        let p1 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let p2 = Poly { coeffs: vec![3, 2, 2, 1, 4], };
        let res = p1 == p2;
        assert_eq!(res, true);
    }

}
