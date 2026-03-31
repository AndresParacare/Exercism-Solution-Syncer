pub fn collatz(n: u64) -> Option<u64> {
    let mut m = n;
    let mut step = 0;
   
   while m > 1 {
       if m % 2 == 0 {
           m /= 2;
           step += 1;
       } else{
           m *= 3;
           m += 1;
           step += 1; 
       }
   }
    if m == 0 {
        None
    } else {
        Some(step)   
    }
}
