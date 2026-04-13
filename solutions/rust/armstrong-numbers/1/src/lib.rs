pub fn is_armstrong_number(num: u32) -> bool {
    let s_num = num.to_string();
    let c_num = s_num.chars().count() as u32;
    let s_num = s_num.chars();
    let mut acum = 0;

    for i in s_num {
        acum += i.to_digit(10).unwrap().pow(c_num);
    }
    
    if acum == num { true } else { false }
}
