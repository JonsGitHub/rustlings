// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..num+1).fold(1, |acc, x|  {
        // dbg!(acc);
        // dbg!(x);
        // dbg!(acc * x)
        acc * x
    })
    
    //From 1 to num, apply fold (accumulates every element that fold is called on, and
    // applies an operation to those numbers). For example from 1 to 4 (should be 24):
    //      (1*1) = 1 -> 
    //      (1*2) = 2 -> 
    //      (2*3) = 6 -> 
    //      (6*4) = 24 -> 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(6, factorial(3));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
