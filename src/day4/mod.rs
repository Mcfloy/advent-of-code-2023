use std::collections::HashMap;
use std::fs;

pub fn part1(path: &str) -> i32 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut result = 0;
  for line in content.lines() {
    let line = line.replace("Card", "");
    let parts: Vec<&str> = line.split(":").collect();
    // let card_number = String::from(parts[0].trim());
    // let card_number = card_number.parse::<i32>().unwrap();
    let cards_section = String::from(parts[1]).replace("  ", " ");
    let cards_section: Vec<&str> = cards_section.split(" | ").collect();
    let mut winning_cards: Vec<i32> = Vec::new();
    for winning_card in cards_section[0].trim().split(" ") {
      let winning_card = winning_card.trim().parse::<i32>().unwrap();
      winning_cards.push(winning_card);
    }
    let mut numbers: Vec<i32> = Vec::new();
    for number in cards_section[1].trim().split(" ") {
      let number = number.trim().parse::<i32>().unwrap();
      numbers.push(number);
    }

    let mut score = 0;
    for number in numbers {
      let winning_card_iter = winning_cards.iter();
      for winning_card in winning_card_iter {
        if *winning_card == number {
          if score == 0 {
            score = 1;
          } else {
            score *= 2;
          }
        }
      }
    }
    result += score;
  }

  result
}

pub fn part2(path: &str) -> i32 {
  let content = fs::read_to_string(path).expect("Unable to read file");

  let mut result = 0;
  let mut copies_of_scratch_card: HashMap<i32, i32> = HashMap::new();
  for line in content.lines() {
    let line = line.replace("Card", "");
    let parts: Vec<&str> = line.split(":").collect();
    let card_number = String::from(parts[0].trim());
    let card_number = card_number.parse::<i32>().unwrap();
    let cards_section = String::from(parts[1]).replace("  ", " ");
    let cards_section: Vec<&str> = cards_section.split(" | ").collect();
    let mut winning_cards: Vec<i32> = Vec::new();
    for winning_card in cards_section[0].trim().split(" ") {
      let winning_card = winning_card.trim().parse::<i32>().unwrap();
      winning_cards.push(winning_card);
    }
    let mut numbers: Vec<i32> = Vec::new();
    for number in cards_section[1].trim().split(" ") {
      let number = number.trim().parse::<i32>().unwrap();
      numbers.push(number);
    }

    let mut number_of_copies = 0;
    for number in numbers {
      let winning_card_iter = winning_cards.iter();
      for winning_card in winning_card_iter {
        if *winning_card == number {
          number_of_copies += 1;
        }
      }
    }

    if !copies_of_scratch_card.contains_key(&card_number) {
      copies_of_scratch_card.insert(card_number, 1);
    }

    for _ in 0..*copies_of_scratch_card.get(&card_number).unwrap_or(&1) {
      for i in (card_number + 1)..(card_number + number_of_copies + 1) {
        let nb_copies = copies_of_scratch_card.get(&i).unwrap_or(&1);
        copies_of_scratch_card.insert(i, nb_copies + 1);
      }
    }
  }

  for value in copies_of_scratch_card.values() {
    result += value;
  }

  result
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1("src/day4/test-part1.txt"), 13);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2("src/day4/test-part2.txt"), 30);
  }
}