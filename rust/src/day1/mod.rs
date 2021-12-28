use queues::{IsQueue, Queue, queue};

use crate::util::load;

fn input() -> Vec<u32> {
    let depths: Vec<u32> = load("data/day1.txt");
    depths
}

fn count_increases(depths: Vec<u32>, win_sz: u32) -> u32 {
    let mut cnt = 0;
    let mut prev = 0;
    let mut sz = 0;
    let mut fifo: Queue<&u32> = queue![];
    for d in depths.iter() {
        if sz < win_sz {
            prev += d;
            sz += 1;
            fifo.add(d).unwrap();
        } else {
            let new = prev + d - fifo.peek().unwrap();
            if new > prev {
                cnt += 1;
            }
            prev = new;
            fifo.remove().unwrap();
            fifo.add(d).unwrap();
        }
    }
    cnt
}

fn part1(depths: Vec<u32>) -> u32 {
    count_increases(depths, 1)
}

fn part2(depths: Vec<u32>) -> u32 {
    count_increases(depths, 3)
}

#[cfg(test)]
mod tests {
    use crate::day1::{input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1722);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1748);
    }
}