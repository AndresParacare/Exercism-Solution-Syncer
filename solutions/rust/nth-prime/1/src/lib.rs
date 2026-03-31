pub fn nth(n: u32) -> u32 { 
    let mut prime = 2;
    let mut k = 3;
    let mut is_prime = false;
    let mut n = n;

    while n != 0{
        for i in 2..k {
            if k % i == 0 {
                is_prime = false;
                break;
            } else {
                is_prime = true;
            }
        }
        if is_prime {
            prime = k;
            n -= 1;
        }
        k = k + 1;
    }
    prime
}
