pub fn raindrops(n: u32) -> String {
    let mut ms = String::new();
   
    if n % 3 == 0 {
       ms.push_str("Pling");
   }
    if n % 5 == 0 {
       ms.push_str("Plang");
   }
    if n % 7 == 0 {
       ms.push_str("Plong");
   }
    if ms.is_empty() {
       ms.push_str(&n.to_string());
   }
    ms
}
