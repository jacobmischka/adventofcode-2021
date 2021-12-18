use adventofcode_2021::get_input;

use std::{fmt, ops::Add};

fn main() {
    let s = get_input().unwrap();
    let mut numbers: Vec<Number> = s
        .lines()
        .rev()
        .map(|line| Number(line.chars().map(NumberElement::from).collect()))
        .collect();

    let mut pairs: Vec<(Number, Number)> = Vec::new();
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                pairs.push((numbers[i].clone(), numbers[j].clone()));
            }
        }
    }

    let mut number = numbers.pop().unwrap();

    while !numbers.is_empty() {
        let next = numbers.pop().unwrap();
        number = number + next;
    }

    let pair = Pair::from_elements(&number.0);
    println!("Part 1: {}", pair.magnitude());

    let mut max = 0;
    for (lhs, rhs) in pairs {
        max = max.max((rhs + lhs).magnitude());
    }

    println!("Part 2: {}", max);
}

#[derive(Debug, Clone)]
struct Number(Vec<NumberElement>);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.0 {
            match element {
                NumberElement::Number(n) => {
                    write!(f, "{}", n)?;
                }
                NumberElement::Open => {
                    write!(f, "[")?;
                }
                NumberElement::Close => {
                    write!(f, "]")?;
                }
                NumberElement::Comma => {
                    write!(f, ",")?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NumberElement {
    Number(u32),
    Open,
    Close,
    Comma,
}

impl From<char> for NumberElement {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => NumberElement::Number(c.to_digit(10).unwrap()),
            '[' => NumberElement::Open,
            ']' => NumberElement::Close,
            ',' => NumberElement::Comma,
            _ => panic!(),
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(mut self, mut rhs: Number) -> Self::Output {
        let mut new = vec![NumberElement::Open];

        new.append(&mut self.0);
        new.push(NumberElement::Comma);
        new.append(&mut rhs.0);

        new.push(NumberElement::Close);

        let mut n = Number(new);
        n.reduce();
        n
    }
}

impl Number {
    fn magnitude(&self) -> u64 {
        Pair::from_elements(&self.0).magnitude()
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn split(&mut self) -> bool {
        let elements = &mut self.0;
        for i in 0..elements.len() {
            if let NumberElement::Number(n) = elements[i] {
                if n > 9 {
                    let half = n as f32 / 2.0;
                    let first = NumberElement::Number(half.floor() as u32);
                    let second = NumberElement::Number(half.ceil() as u32);

                    elements[i] = NumberElement::Close;
                    elements.insert(i, second);
                    elements.insert(i, NumberElement::Comma);
                    elements.insert(i, first);
                    elements.insert(i, NumberElement::Open);
                    return true;
                }
            }
        }

        false
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for i in 0..self.0.len() {
            match self.0[i] {
                NumberElement::Open => {
                    if depth == 4 {
                        if let (NumberElement::Number(first), NumberElement::Number(second)) =
                            (self.0[i + 1], self.0[i + 3])
                        {
                            let mut j = i - 1;
                            while j > 0 {
                                if let NumberElement::Number(n) = self.0[j] {
                                    self.0[j] = NumberElement::Number(n + first);
                                    break;
                                }
                                j -= 1;
                            }
                            j = i + 5;
                            while j < self.0.len() {
                                if let NumberElement::Number(n) = self.0[j] {
                                    self.0[j] = NumberElement::Number(n + second);
                                    break;
                                }
                                j += 1;
                            }
                            self.0.remove(i);
                            self.0.remove(i);
                            self.0.remove(i);
                            self.0.remove(i);
                            self.0.remove(i);
                            self.0.insert(i, NumberElement::Number(0));
                            return true;
                        } else {
                            panic!("Invalid exploding pair: {:?}", &self.0[i..=i + 4]);
                        }
                    } else {
                        depth += 1;
                    }
                }
                NumberElement::Close => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct Pair(PairElement, PairElement);

#[derive(Debug, Clone)]
enum PairElement {
    Number(u32),
    Pair(Box<Pair>),
}

impl PairElement {
    fn magnitude(&self) -> u64 {
        match self {
            PairElement::Number(x) => *x as u64,
            PairElement::Pair(p) => p.magnitude(),
        }
    }
}

impl Pair {
    fn magnitude(&self) -> u64 {
        3 * self.0.magnitude() + 2 * self.1.magnitude()
    }
}

impl Pair {
    fn from_elements(elements: &[NumberElement]) -> Self {
        let mut i = 1;
        let first = match elements[1] {
            NumberElement::Open => {
                let mut stack = Vec::new();
                while i < elements.len() {
                    match elements[i] {
                        NumberElement::Open => stack.push(elements[i]),
                        NumberElement::Close => {
                            stack.pop();
                            if stack.is_empty() {
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                i += 1;
                PairElement::Pair(Box::new(Pair::from_elements(&elements[1..i])))
            }
            NumberElement::Number(n) => {
                i += 1;
                PairElement::Number(n)
            }
            _ => panic!(),
        };
        i += 1;
        let mut j = i;
        let second = match elements[j] {
            NumberElement::Open => {
                let mut stack = Vec::new();
                while j < elements.len() {
                    match elements[j] {
                        NumberElement::Open => stack.push(elements[j]),
                        NumberElement::Close => {
                            stack.pop();
                            if stack.is_empty() {
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                PairElement::Pair(Box::new(Pair::from_elements(&elements[i..=j])))
            }
            NumberElement::Number(n) => PairElement::Number(n),
            _ => panic!("Invalid second: {:?}: {:?}", &elements, &elements[j]),
        };

        Pair(first, second)
    }
}
