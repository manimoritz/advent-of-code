fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

const CARDS: [&str; 13] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

fn part1(input: &str) -> u32 {
    let mut five_of_a_kind = vec![];
    let mut four_of_a_kind = vec![];
    let mut full_house = vec![];
    let mut three_of_a_kind = vec![];
    let mut two_pair = vec![];
    let mut one_pair = vec![];
    let mut high_card = vec![];

    let mut hand_bid_pairs = vec![];
    for line in input.lines() {
        let tmp = line.split_ascii_whitespace().collect::<Vec<&str>>();
        hand_bid_pairs.push((tmp[0], tmp[1].parse::<u32>().unwrap()));
    }
    for (hand, _) in &hand_bid_pairs {
        let mut largest_card_count = 0;
        let mut two_or_three = false;
        let mut second_pair = false;
        for card in CARDS {
            let tmp = hand.matches(card).count();
            if largest_card_count < tmp {
                largest_card_count = tmp;
                if tmp == 2 || tmp == 3 {
                    two_or_three = true;
                }
            }
            if two_or_three {
                let cards_without = CARDS.iter().filter(|x| **x != card);
                for second_card in cards_without {
                    if hand.matches(second_card).count() == 2 {
                        second_pair = true;
                    }
                }
            }
            two_or_three = false
        }
        if largest_card_count == 5 {
            five_of_a_kind.push(*hand);
        } else if largest_card_count == 4 {
            four_of_a_kind.push(*hand);
        } else if largest_card_count == 3 {
            if second_pair {
                full_house.push(*hand);
            } else {
                three_of_a_kind.push(*hand);
            }
        } else if largest_card_count == 2 {
            if second_pair {
                two_pair.push(*hand);
            } else {
                one_pair.push(*hand);
            }
        } else {
            high_card.push(*hand);
        }
    }
    // dbg!(&five_of_a_kind);
    // dbg!(&four_of_a_kind);
    // dbg!(&full_house);
    // dbg!(&three_of_a_kind);
    // dbg!(&two_pair);
    // dbg!(&one_pair);
    // dbg!(&high_card);

    let sorted_five_of_a_kind = sort(five_of_a_kind);
    // dbg!(&sorted_five_of_a_kind);
    let sorted_four_of_a_kind = sort(four_of_a_kind);
    // dbg!(&sorted_four_of_a_kind);
    let sorted_full_house = sort(full_house);
    // dbg!(&sorted_full_house);
    let sorted_three_of_a_kind = sort(three_of_a_kind);
    // dbg!(&sorted_three_of_a_kind);
    let sorted_two_pair = sort(two_pair);
    // dbg!(&sorted_two_pair);
    let sorted_one_pair = sort(one_pair);
    // dbg!(&sorted_one_pair);
    let sorted_high_card = sort(high_card);
    // dbg!(&sorted_high_card);

    let whole_list = [
        sorted_high_card,
        sorted_one_pair,
        sorted_two_pair,
        sorted_three_of_a_kind,
        sorted_full_house,
        sorted_four_of_a_kind,
        sorted_five_of_a_kind,
    ]
    .concat();
    // dbg!(&whole_list);
    let mut bid_sum = 0;
    for (i, hand1) in whole_list.iter().enumerate() {
        // dbg!(&hand1);
        for (hand, value) in &hand_bid_pairs {
            // dbg!(&hand);
            if hand1 == hand {
                bid_sum += value * ((i + 1) as u32);
                // dbg!(bid_sum);
            }
        }
    }
    bid_sum
}

fn sort(mut hands: Vec<&str>) -> Vec<&str> {
    let len = hands.len();
    for i in 0..len {
        for j in i..len {
            for k in 0..5 {
                let char_rank_1 = 13 - search_char(hands[i], k);
                let char_rank_2 = 13 - search_char(hands[j], k);
                if char_rank_1 < char_rank_2 {
                    break;
                } else if char_rank_1 > char_rank_2 {
                    let tmp = hands[i];
                    hands[i] = hands[j];
                    hands[j] = tmp;
                    break;
                }
            }
        }
    }
    return hands;
}

fn search_char(hand: &str, counter: usize) -> usize {
    let mut c_pos = std::usize::MAX;
    for i in 0..13 {
        if &hand[counter..counter + 1] == CARDS[i] {
            c_pos = i;
        }
    }
    c_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_case = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(test_case);
        assert_eq!(result, 6440u32);
    }
}
