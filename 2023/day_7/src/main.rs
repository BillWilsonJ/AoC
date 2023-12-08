use std::fs;
use std::collections::BTreeMap;

fn main() {
    let contents = fs::read_to_string("src\\input.txt")
    .expect("Should have been able to read the file");

    let contents_clone = contents.clone();
    part_1_sln(contents);
    part_2_sln(contents_clone);
}

fn part_1_sln(input: String) {
    let mut hands = BTreeMap::new();
    for line in input.lines() {
        let new_cards = line
            .split(" ")
            .nth(0)
            .unwrap();

        let hand = Hand {
            cards: new_cards,

            bid: line
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<i32>().unwrap(),

            score: determine_hand_score(new_cards)
        };
        hands.insert(hand,0);
    }

    let mut part_1_answer = 0;
    let mut index = 1;
    for (hand,_idk) in hands {
        part_1_answer += hand.bid * index;
        index+=1;
    }

    println!("Part 1 Answer: {}",part_1_answer);
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: i32,
    score: HandScore,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score != other.score {
            self.score.cmp(&other.score)
        } else {
            for (self_char, other_char) in self.cards.chars().zip(other.cards.chars()) {
                if self_char != other_char {
                    return char_to_value(self_char).cmp(&char_to_value(other_char));
                }
            }
            panic!("None should be equal");
        }
    }
}

fn char_to_value(x: char) -> u8 {
    match x {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unexpected char"),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandScore {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7
}

fn determine_hand_score(hand: &str) -> HandScore {

    let mut hand_score = HandScore::HighCard;
    let mut match_found = false;
    let mut match_found_value = 0;
    let mut match_found_char: char = '-';

    for i in 0..hand.len() {
        let current_card = hand.chars().nth(i).unwrap();
        let mut num_matches = 0;
        for n in  i..hand.len(){
            let new_card = hand.chars().nth(n).unwrap();
            if current_card == match_found_char {
                break;
            }
            if current_card == new_card {
                num_matches+=1;
            }
        }

        if num_matches == 5 {
            hand_score = HandScore::FiveOfAKind;
            break;
        }
        else if num_matches == 4 {
            hand_score = HandScore::FourOfAKind;
            break;
        }
        else if num_matches == 3 {
            if match_found == false {
                hand_score = HandScore::ThreeOfAKind;
                if i < 4 {
                    match_found = true;
                    match_found_value = 3;
                    match_found_char = current_card;
                }
                else {
                    break;
                }
            }
            else {
                hand_score = HandScore::FullHouse;
                break;
            }
        } else if num_matches == 2 {
            if match_found == false {
                hand_score = HandScore::OnePair;
                if i < 4 {
                    match_found = true;
                    match_found_value = 2;
                    match_found_char = current_card;
                }
                else {
                    break;
                }
            }
            else {
                if match_found_value == 2 {
                    hand_score = HandScore::TwoPair;
                    break;
                }
                else {
                    hand_score = HandScore::FullHouse;
                    break;
                }
            }
        }
    }

    return hand_score;
}

// Part 2
#[derive(Debug, PartialEq, Eq)]
struct Hand2<'a> {
    cards: &'a str,
    bid: i32,
    score: HandScore,
}

impl<'a> PartialOrd for Hand2<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand2<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score != other.score {
            self.score.cmp(&other.score)
        } else {
            for (self_char, other_char) in self.cards.chars().zip(other.cards.chars()) {
                if self_char != other_char {
                    return char_to_value2(self_char).cmp(&char_to_value2(other_char));
                }
            }
            panic!("None should be equal");
        }
    }
}

fn char_to_value2(x: char) -> u8 {
    match x {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Unexpected char"),
    }
}

fn determine_hand_score2(hand: &str) -> HandScore {

    let mut hand_score = HandScore::HighCard;
    let mut match_found = false;
    let mut match_found_value = 0;
    let mut match_found_char: char = '-';
    let mut j_score_used = false;
    let mut compare_string: &str = hand;
    let mut num_of_j = 0;
    
    for char in hand.chars() {
        if char == 'J' {
            num_of_j+=1;
        }
    }

    let mut new_string = compare_string.replace("J", "");

    for _n in 0..num_of_j {
        new_string.push('J');
    }

    compare_string = new_string.as_str();

    for i in 0..compare_string.len() {
        let current_card = compare_string.chars().nth(i).unwrap();
        let mut num_matches = 0;
        let mut j_score_current = false;
        for n in  i..compare_string.len(){
            let new_card = compare_string.chars().nth(n).unwrap();
            if current_card == match_found_char {
                break;
            }
            else if current_card == new_card {
                num_matches+=1;
            }
            else if num_matches != 0 && new_card == 'J' {
                num_matches+=1;
                j_score_current = true;
            }
        }

        if num_matches == 5 {
            hand_score = HandScore::FiveOfAKind;
            break;
        }
        else if num_matches == 4 {
            hand_score = HandScore::FourOfAKind;
            break;
        }
        else if num_matches == 3 {
            if j_score_current {
                j_score_used = true;
            }
            if match_found == false {
                hand_score = HandScore::ThreeOfAKind;
                if i < 4 {
                    match_found = true;
                    match_found_value = 3;
                    match_found_char = current_card;
                }
                else {
                    break;
                }
            }
            else if num_of_j == 1 && j_score_current && j_score_used && match_found_value == 2{
                hand_score = HandScore::ThreeOfAKind;
            }
            else if num_of_j == 1 && j_score_current && j_score_used{
                hand_score = HandScore::FullHouse;
            }
            else if num_of_j == 1 && j_score_current {
                hand_score = HandScore::ThreeOfAKind;
            }
            else if num_of_j == 2 && j_score_current {
                hand_score = HandScore::ThreeOfAKind;
                break;
            }
            else {
                hand_score = HandScore::FullHouse;
                break;
            }
        } else if num_matches == 2 {
            if j_score_current {
                j_score_used = true;
            }
            if match_found == false {
                hand_score = HandScore::OnePair;
                if i < 4 {
                    match_found = true;
                    match_found_value = 2;
                    match_found_char = current_card;
                }
                else {
                    break;
                }
            }
            else if num_of_j == 1 && j_score_current  {
                if hand_score == HandScore::ThreeOfAKind {
                    break;
                }
            }
            else if num_of_j == 2 && j_score_current {
                hand_score = HandScore::ThreeOfAKind;
                break;
            }
            else {
                if match_found_value == 2 {
                    hand_score = HandScore::TwoPair;
                    break;
                }
                else if num_of_j == 2 {
                    hand_score = HandScore::ThreeOfAKind;
                    break;
                }
                else {
                    hand_score = HandScore::FullHouse;
                    break;
                }
            }
        }
    }

    return hand_score;
}

fn part_2_sln(input: String) {
    let mut hands = BTreeMap::new();
    for line in input.lines() {
        let new_cards = line
            .split(" ")
            .nth(0)
            .unwrap();

        let hand = Hand2 {
            cards: new_cards,

            bid: line
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<i32>().unwrap(),

            score: determine_hand_score2(new_cards)
        };
        hands.insert(hand,0);
    }

    let mut part_2_answer = 0;
    let mut index = 1;
    for (hand,_idk) in hands {
        part_2_answer += hand.bid * index;
        index+=1;
    }

    println!("Part 2 Answer: {}",part_2_answer);
}