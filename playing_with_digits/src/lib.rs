
pub fn dig_pow(n: i64, p: i32) -> i64 {
    let mut exp: i32 = p;
    let mut sum: i64 = 0;

    for d in n.to_string().chars() {
        sum += (d.to_digit(10).unwrap() as u64).pow(exp as u32) as i64;
        exp += 1;
    }

    if sum % n == 0 { sum / n } else { -1 } 
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
        
    }
}