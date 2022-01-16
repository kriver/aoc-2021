const X_ADD: [i64; 14] = [13, 12, 11, 0, 15, -13, 10, -9, 11, 13, -14, -3, -2, -14];
const Z_DIV: [i64; 14] = [1, 1, 1, 26, 1, 26, 1, 26, 1, 1, 26, 26, 26, 26];
const Y_ADD: [i64; 14] = [14, 8, 5, 4, 10, 13, 16, 5, 6, 13, 6, 7, 13, 3];

fn solve_1(inp: &Vec<u8>) -> i64 {
    let mut z = 0;
    for (i, digit) in inp.iter().enumerate() {
        if z % 26 == *digit as i64 - X_ADD[i] {
            z /= 26;
        } else {
            z /= Z_DIV[i];
            z = z * 26 + *digit as i64 + Y_ADD[i];
        }
    }
    z
}

fn solve(inp: &mut Vec<u8>, start: usize) -> (Vec<u8>, i64) {
    if start == 14 {
        return (inp.to_vec(), solve_1(inp));
    }
    let mut minimum = i64::MAX;
    for i in start..14 {
        minimum = i64::MAX;
        let mut min_x = 0;
        for x in 1..10 {
            inp[i] = x;
            let s = solve_1(inp);
            if s < minimum {
                minimum = s;
                min_x = x;
            }
        }
        inp[i] = min_x;
    }
    (inp.to_vec(), minimum)
}

fn run<F>(range_supplier: F) -> i64
    where F: Fn(usize) -> Vec<u8>
{
    let ones = vec![1u8; 14];
    let mut inp = vec![1u8; 14];
    let (mut sol, mut minimum) = solve(&mut inp, 0);
    for i in 0..14 {
        for d in range_supplier(sol[i] as usize).into_iter() {
            inp = sol[0..i + 1].to_vec();
            inp.extend_from_slice(&ones[i + 1..]);
            inp[i] = d;
            let (_inp, maybe_min) = solve(&mut inp, i + 1);
            if maybe_min <= minimum {
                sol[i] = d;
                minimum = maybe_min;
            }
        }
    }
    sol.into_iter().fold(0, |acc, d| acc * 10 + d as i64)
}

fn part1() -> i64 {
    run(|d| (d + 1..10).map(|v| v as u8).collect())
}

fn part2() -> i64 {
    run(|d| (1..=d - 1).rev().map(|v| v as u8).collect())
}

#[cfg(test)]
mod tests {
    use crate::day24::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 93499629698999);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 11164118121471);
    }
}
