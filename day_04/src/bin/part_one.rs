use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
struct Card<'a> {
    self_numbers: Vec<&'a str>,
    winning_numbers: Vec<&'a str>,
}
impl<'a> Card<'a> {
    fn score(&self) -> u32 {
        let matches = self
            .self_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32;
        match matches.checked_sub(1) {
            Some(num) => 2u32.pow(num),
            None => 0,
        }
    }
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> u32 {
    let (_, cards) = parser(input).unwrap();
    cards.iter().map(|card| card.score()).sum()
}

fn parser(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    let (input, sides) = separated_list1(tuple((space1, tag("|"), space1)), side)(input)?;
    Ok((
        input,
        Card {
            self_numbers: sides[0].clone(),
            winning_numbers: sides[1].clone(),
        },
    ))
}

fn side(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, digit1)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = run(input);
        assert_eq!(13, result)
    }
}
