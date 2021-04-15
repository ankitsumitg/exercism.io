/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digs:Vec<i32> = Vec::new();
    for c in code.replace(" ","").chars(){
        if c.is_digit(10){
            digs.push(c.to_digit(10).unwrap() as i32);
        }
        else{
            return false
        }
    }
    digs
    .iter()
    .rev()
    .enumerate()
    .map(|(pos,value)|
    if pos&1 == 1{
        if (*value)*2>9{
            (*value)*2 - 9
        }
        else{
            (*value)*2
        }
    } else {
        *value
    })
    .sum::<i32>()%10 == 0 && digs.len() >1
}
