use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::Add;

use itertools::Itertools;

use NodeType::{Branch, Value};

use crate::util::load;

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Value(u8),
    Branch(Box<Node>, Box<Node>),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value(v) => write!(f, "{}", v),
            Branch(l, r) => write!(f, "[{},{}]", l.node_type, r.node_type)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    depth: usize,
    node_type: NodeType,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node_type)
    }
}

impl From<&str> for Node {
    fn from(line: &str) -> Self {
        fn parse_recursive(chars: &mut impl Iterator<Item=char>, depth: usize) -> Node {
            match chars.next() {
                Some(c) => match c {
                    '[' => Node {
                        depth,
                        node_type: Branch(
                            Box::new(parse_recursive(chars, depth + 1)),
                            Box::new(parse_recursive(chars, depth + 1))),
                    },
                    ']' | ',' => parse_recursive(chars, depth),
                    c
                    if c >= '0' && c <= '9' => Node {
                        depth,
                        node_type: Value(c as u8 - '0' as u8),
                    },
                    _ => unreachable!("line corrupted")
                }
                None => unreachable!("line corrupted")
            }
        }
        parse_recursive(&mut line.chars(), 0)
    }
}

impl Node {
    fn magnitude(&self) -> u32 {
        match &self.node_type {
            Value(n) => *n as u32,
            Branch(l, r) => 3 * l.magnitude() + 2 * r.magnitude()
        }
    }

    fn incr_depth(&mut self) {
        self.depth += 1;
        match &mut self.node_type {
            Value(_) => (),
            Branch(l, r) => {
                l.incr_depth();
                r.incr_depth();
            }
        }
    }

    fn reduce(&mut self) {
        enum Reduction {
            None,
            Exploded(Option<u8>, Option<u8>),
            Split,
        }

        fn add_right(n: &mut Node, delta: u8) {
            match &mut n.node_type {
                Branch(_l, r) => add_right(r, delta),
                Value(v) => *v += delta,
            };
        }

        fn add_left(n: &mut Node, delta: u8) {
            match &mut n.node_type {
                Branch(l, _r) => add_left(l, delta),
                Value(v) => *v += delta,
            };
        }

        fn reduce_explode(n: &mut Node) -> Reduction {
            match &mut n.node_type {
                Branch(l, r) if n.depth == 4 => {
                    if let (Value(lv), Value(rv)) = (&l.node_type, &r.node_type) {
                        let reduction = Reduction::Exploded(Some(*lv), Some(*rv));
                        n.node_type = Value(0);
                        reduction
                    } else {
                        panic!("invalid node {:?}", n)
                    }
                }
                Branch(l, r) => {
                    match reduce_explode(l) {
                        Reduction::None => match reduce_explode(r) {
                            Reduction::Exploded(Some(lv), rv) => {
                                add_right(l, lv);
                                Reduction::Exploded(None, rv)
                            }
                            other => other,
                        }
                        Reduction::Exploded(lv, Some(rv)) => {
                            add_left(r, rv);
                            Reduction::Exploded(lv, None)
                        }
                        other => other
                    }
                }
                Value(_v) => Reduction::None
            }
        }

        fn reduce_split(n: &mut Node) -> Reduction {
            match &mut n.node_type {
                Branch(l, r) => {
                    match reduce_split(l) {
                        Reduction::None => reduce_split(r),
                        other => other
                    }
                }
                Value(v) if *v > 9 => {
                    n.node_type = Branch(
                        Box::new(Node {
                            depth: n.depth + 1,
                            node_type: Value(*v / 2),
                        }),
                        Box::new(Node {
                            depth: n.depth + 1,
                            node_type: Value(*v - *v / 2),
                        }),
                    );
                    Reduction::Split
                }
                Value(_v) => Reduction::None
            }
        }

        loop {
            if let Reduction::None = reduce_explode(self) {
                if let Reduction::None = reduce_split(self) {
                    break;
                }
            }
        }
    }
}

impl Add for Node {
    type Output = Node;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.incr_depth();
        rhs.incr_depth();
        let mut sum = Node {
            depth: 0,
            node_type: Branch(Box::new(self), Box::new(rhs)),
        };
        sum.reduce();
        sum
    }
}

impl Sum for Node {
    fn sum<I: Iterator<Item=Self>>(mut iter: I) -> Self {
        match iter.next() {
            None => panic!("unable to sum empty list"),
            Some(first) => iter.fold(first, |a, b| a + b)
        }
    }
}

fn input() -> Vec<Node> {
    let lines: Vec<String> = load("data/day18.txt");
    lines.into_iter()
        .map(|line| Node::from(line.as_str()))
        .collect()
}

fn part1(numbers: Vec<Node>) -> u32 {
    numbers.into_iter()
        .sum::<Node>()
        .magnitude()
}


fn part2(numbers: Vec<Node>) -> u32 {
    numbers.into_iter()
        .permutations(2)
        .map(|mut v| v.remove(0) + v.remove(0))
        .map(|n| n.magnitude())
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day18::{input, Node, part1, part2};

    #[test]
    fn test_node_magnitude() {
        assert_eq!(Node::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
                       .magnitude(),
                   4140);
    }

    #[test]
    fn test_add_reduce() {
        let a = Node::from("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = Node::from("[1,1]");
        let sum = a + b;
        assert_eq!(format!("{}", sum), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }

    #[test]
    fn test_reduce() {
        let mut n = Node::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(format!("{}", n), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 2907);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4690);
    }
}
