pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut m: u64 = n;
    let mut acum: u64 = 2;
    
    while m > 1 {
        if m % acum == 0 {
            factors.push(acum);
            m = m / acum;
            acum = 2;
        } else {
            acum += 1;
        }
    }
    factors
}
