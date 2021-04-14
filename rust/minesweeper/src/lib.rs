static NEIGHBOURS:&[(i32,i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let h = minefield.len() as i32;
    (0..h).map(|y| {
        let w = minefield[0 as usize].len() as i32;
        (0..w).map(|x| {
            if minefield[y as usize].as_bytes()[x as usize] == b'*'{
                '*'
            }
            else{
                match NEIGHBOURS.iter()
                .map(|&(checkx,checky)| (x +  checkx, y + checky))
                .filter(|&(x,y)| (0 <= x && x < w) && (0 <= y && y < h) && minefield[y as usize].as_bytes()[x as usize] == b'*')
                .count()
                {
                    0 => ' ',
                    n => (n as u8 + '0' as u8) as char
                }
            }
        }).collect()
    }).collect()
}
