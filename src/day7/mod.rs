use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use log::debug;

#[derive(PartialEq, Clone, Copy)]
pub enum Evaluation {
  PartOne,
  PartTwo,
}

fn get_value(c: char, evaluation: Evaluation) -> u64 {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => match evaluation {
      Evaluation::PartOne => 11,
      Evaluation::PartTwo => 1,
    }
    'T' => 10,
    _ => c.to_digit(10).unwrap() as u64,
  }
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Debug)]
enum HandType {
  FiveOfAKind = 8,
  FourOfAKind = 7,
  FullHouse = 6,
  ThreeOfAKind = 5,
  TwoPairs = 4,
  OnePair = 3,
  HighCard = 2,
}

impl Ord for HandType {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

pub struct Cards {
  full_value: String,
  r#type: HandType,
}

impl Cards {
  pub fn new(value: &str, evaluation: Evaluation) -> Cards {
    let mut card_type = HashMap::new();
    for c in value.chars() {
      card_type.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut kind = HandType::HighCard;
    for (k, v) in card_type.iter() {
      if evaluation == Evaluation::PartTwo && *k == 'J' {
        continue;
      }
      if *v == 5 {
        kind = HandType::FiveOfAKind;
        break;
      }
      if *v == 4 {
        kind = HandType::FourOfAKind;
        break;
      }
      if *v == 3 {
        if kind == HandType::OnePair {
          kind = HandType::FullHouse;
        } else {
          kind = HandType::ThreeOfAKind;
        }
      }
      if *v == 2 {
        if kind == HandType::ThreeOfAKind {
          kind = HandType::FullHouse;
        } else if kind == HandType::OnePair {
          kind = HandType::TwoPairs;
        } else {
          kind = HandType::OnePair;
        }
      }
    }

    if evaluation == Evaluation::PartTwo {
      match card_type.get(&'J') {
        Some(v) => {
          if *v == 1 {
            match kind {
              HandType::FourOfAKind => kind = HandType::FiveOfAKind,
              HandType::ThreeOfAKind => kind = HandType::FourOfAKind,
              HandType::TwoPairs => kind = HandType::FullHouse,
              HandType::OnePair => kind = HandType::ThreeOfAKind,
              HandType::HighCard => kind = HandType::OnePair,
              _ => {}
            }
          }
          if *v == 2 {
            match kind {
              HandType::ThreeOfAKind => kind = HandType::FiveOfAKind,
              HandType::OnePair => kind = HandType::FourOfAKind,
              HandType::HighCard => kind = HandType::ThreeOfAKind,
              _ => {}
            }
          }
          if *v == 3 {
            match kind {
              HandType::OnePair => kind = HandType::FiveOfAKind,
              HandType::HighCard => kind = HandType::FourOfAKind,
              _ => {}
            }
          }
          if *v == 4 {
            match kind {
              HandType::HighCard => kind = HandType::FiveOfAKind,
              _ => {}
            }
          }
          if *v == 5 {
            match kind {
              HandType::HighCard => kind = HandType::FiveOfAKind,
              _ => {}
            }
          }
        }
        None => {}
      }
    }


    Cards {
      full_value: value.to_string(),
      r#type: kind,
    }
  }

  // True if the current hand is better than the other hand
  pub fn cmp(&self, other: &Cards, evaluation: Evaluation) -> Ordering {
    if self.r#type.cmp(&other.r#type) == Ordering::Equal {
      // Compare each card
      for (c1, c2) in self.full_value.chars().zip(other.full_value.chars()) {
        let v1 = get_value(c1, evaluation);
        let v2 = get_value(c2, evaluation);
        if v1 != v2 {
          return v1.cmp(&v2);
        }
      }
      panic!("Unexpected equal hands")
    } else {
      self.r#type.cmp(&other.r#type)
    }
  }
}

pub struct Hand {
  cards: Cards,
  bid: u64,
  rank: u64,
}

impl Hand {
  pub fn from_line(line: &str, evaluation: Evaluation) -> Hand {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let cards = parts[0];
    let bid = parts[1].parse::<u64>().unwrap();

    Hand {
      cards: Cards::new(cards, evaluation),
      bid,
      rank: 0,
    }
  }
}

pub fn part1(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let lines = content.lines();
  let mut hands: Vec<Hand> = Vec::new();
  for line in lines {
    hands.push(Hand::from_line(line, Evaluation::PartOne));
  }

  hands.sort_by(|a, b| a.cards.cmp(&b.cards, Evaluation::PartOne));

  for (i, hand) in hands.iter_mut().enumerate() {
    hand.rank = i as u64 + 1;
    debug!("{} {} {:?} {}", hand.rank, hand.cards.full_value, hand.cards.r#type, hand.bid);
  }

  let mut result = 0;
  for hand in hands {
    result += hand.bid * hand.rank;
  }

  result
}

pub fn part2(path: &str) -> u64 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let lines = content.lines();
  let mut hands: Vec<Hand> = Vec::new();
  for line in lines {
    hands.push(Hand::from_line(line, Evaluation::PartTwo));
  }

  hands.sort_by(|a, b| a.cards.cmp(&b.cards, Evaluation::PartTwo));

  for (i, hand) in hands.iter_mut().enumerate() {
    hand.rank = i as u64 + 1;
    debug!("{} {} {:?} {}", hand.rank, hand.cards.full_value, hand.cards.r#type, hand.bid);
  }

  let mut result = 0;
  for hand in hands {
    result += hand.bid * hand.rank;
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day7/test-part1.txt"), 6440);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day7/test-part2.txt"), 5905);
  }
}