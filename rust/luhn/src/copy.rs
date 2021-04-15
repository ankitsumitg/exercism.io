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
    if digs.len()%2 == 1{
        digs.insert(0,0);
    }
    let mut sum =0;
    for (pos,val) in digs.iter().enumerate(){
        let mut v: i32 = *val;
        if pos%2 == 0{
            v = v*2;
            if v > 9{
                v -= 9;
            }
        }
        sum += v;
    }
    sum%10 == 0 && !{digs.len() == 2 && sum == 0}
}
