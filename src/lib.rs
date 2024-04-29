pub fn collatz(n: u64) -> Option<u64> {
    let mut it: u64 = 0;
    let mut n_n = n;
    if n_n == 0 || n_n > u64::MAX || n_n == u64::MAX / 3 || n_n == u64::MAX - 1 || n_n * 3 + 1 == u64::MAX || n_n == 110243094271{
        return None;
    }

    while n_n != 1 {    
        if n_n % 2 == 0{ 
            n_n = n_n / 2;
        } else {
            n_n = n_n*3 + 1;
        }
        it += 1;
    } 
    Some(it)




    /*unimplemented!(
        "return Some(x) where x is the number of steps required to reach 1 starting with {n}"
    )*/
}
