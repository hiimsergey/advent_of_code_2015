const INPUT: &str = include_str!("input/2");

pub fn one() {
    let result: u32 = INPUT.lines().map(|line| {
        let lwh: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        let (lw, wh, hl) = (lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]);

        2 * lw + 2 * wh + 2 * hl + lw.min(wh).min(hl)
    }).sum();

    println!("{result}");
}

pub fn two() {
    let result: u32 = INPUT.lines().map(|line| {
        let mut lwh: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        lwh.sort();

        2 * lwh[0] + 2 * lwh[1] + lwh.iter().product::<u32>()
    }).sum();

    println!("{result}");
}
