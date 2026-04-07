#[cfg(test)]
mod tests {
    // we adjust the path to:
    use super::super::{*};

    #[test]
    fn count_peaks_test() {
        assert_eq!(count_peaks(&[2, 1, 0]), 1);
        assert_eq!(count_peaks(&[0, 2, 0, 4, 1]), 2);
        assert_eq!(count_peaks(&[3, 2, 0, 4, 7]), 2);
    }

    #[test]
    fn remove_runs_test() {
        assert_eq!(remove_runs(&[1, 1, 2, 4, 5, 5, 5, 5, 7, 9, 9]), vec![1, 2, 4, 5, 7, 9]);
        assert_eq!(remove_runs(&[1, 1, 2, 2, 1, 1, 2, 2]), vec![1, 2, 1, 2]);
        assert_eq!(remove_runs(&[9, 9, 9, 9, 9, 9, 9, 9, 9]), vec![9]);
        assert_eq!(remove_runs(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(remove_runs(&[]), vec![]);
        assert_eq!(remove_runs(&[42]), vec![42]);
        assert_eq!(remove_runs(&[42]), vec![42]);
    }
      
    #[test]
    fn count_and_remove_primes_test() {
        let mut nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let non_primes = [1, 0, 0, 4, 0, 6, 0, 8, 9, 10, 0, 12, 0, 14, 15, 16, 0, 18, 0, 20];
        assert_eq!(count_and_remove_primes(&mut nums), 8);
        assert_eq!(nums, non_primes);
    }
  
    #[test]
    fn safe_squares_rooks_test() {
        assert_eq!(safe_squares_rooks(10, &[]), 100);
        assert_eq!(safe_squares_rooks(4, &[(2, 3), (0, 1)]), 4);
        assert_eq!(safe_squares_rooks(8, &[(1, 1), (3, 5), (7, 0), (7, 6)]), 20);
        assert_eq!(safe_squares_rooks(2, &[(1, 1)]), 1);
    }

}
