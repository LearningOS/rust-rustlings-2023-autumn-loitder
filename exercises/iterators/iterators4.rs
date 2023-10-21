// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: i64) -> i64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    
    //哎，看了半天看错题了
    // let mut n : i64 = 1;
    // (1..=num).fold(num,|acc,x| {
    //     let tmp = acc / x;
    //     if tmp == 1 {
    //         n = x;
    //         -1
    //     } else if tmp == 0 {
    //         -1
    //     } else if tmp < 0 {
    //         -1
    //     } else {
    //         tmp
    //     }
    // });
    // n


    // (1..=num).product()
    (1..=num).fold(1,|acc,x|{
        x * acc
    })
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
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
