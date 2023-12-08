use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../input/day7").unwrap();
    let mut hands: Vec<(Hand, usize)> = input.lines().map(|l| create_hand(l)).collect();
    hands.sort_by(|(h1, _), (h2, _)| h1.partial_cmp(h2).unwrap());
    hands.reverse();
    for (hand, n) in &hands {
        println!("{} {n}", hand.raw)
    }
    let v = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, n))| acc + ((i + 1) * n));
    println!("P2: {}", v);
}

#[derive(PartialEq, Eq, Debug)]
struct Hand<'a> {
    hand_type: HandType,
    raw: &'a str,
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_cmp != Ordering::Equal {
            Some(hand_cmp)
        } else {
            for (c1, c2) in self.raw.chars().zip(other.raw.chars()) {
                if c1 == c2 {
                    continue;
                }
                let card: Card = c1.into();
                let into = c2.into();
                let cmp = card.partial_cmp(&into).unwrap();
                if cmp != Ordering::Equal {
                    return Some(cmp);
                }
            }
            unreachable!()
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(u8)]
enum Card {
    Special(FaceCard),
    Num(u32),
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum FaceCard {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Num(n1), Self::Num(n2)) => n2.partial_cmp(n1),
            (_, Self::Special(FaceCard::Joker)) => Some(Ordering::Less),
            (Self::Special(FaceCard::Joker), _) => Some(Ordering::Greater),
            (_, Self::Num(_)) => Some(Ordering::Less),
            (Self::Num(_), _) => Some(Ordering::Greater),
            (Self::Special(s1), Self::Special(s2)) => s1.partial_cmp(s2),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Special(FaceCard::Ace),
            'K' => Self::Special(FaceCard::King),
            'Q' => Self::Special(FaceCard::Queen),
            'J' => Self::Special(FaceCard::Joker),
            'T' => Self::Special(FaceCard::Ten),
            c => Self::Num(c.to_digit(10).unwrap()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn create_hand(l: &str) -> (Hand, usize) {
    let (hand, nums) = l.split_once(' ').unwrap();
    let mut total = HashMap::new();
    let mut jokers = 0;
    for c in hand.chars() {
        if c == 'J' {
            jokers += 1;
        } else {
            total.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let mut sorted_chars: Vec<u32> = total.iter().map(|(_k, v)| *v).collect();
    sorted_chars.sort_unstable();
    sorted_chars.reverse();
    if sorted_chars.is_empty() {
        sorted_chars.push(0);
    }
    sorted_chars[0] += jokers;
    let hand_type = match (sorted_chars[0], sorted_chars.get(1)) {
        (5, None) => HandType::FiveOfKind,
        (4, _) => HandType::FourOfKind,
        (3, Some(2)) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfKind,
        (2, Some(2)) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    };
    (
        Hand {
            raw: hand,
            hand_type,
        },
        nums.parse().unwrap(),
    )
}
