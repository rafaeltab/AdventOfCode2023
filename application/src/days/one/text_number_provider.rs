use super::number_provider::NumberProvider;
use lazy_static::lazy_static;
use util::trie::Trie;

pub struct TextNumberProvider {}

impl NumberProvider for TextNumberProvider {
    fn extract_numbers(&self, text: &str) -> Vec<Vec<u32>> {
        let mut i = 0;
        let mut res: Vec<Vec<u32>> = vec![vec![]];
        let chars: Vec<char> = text.chars().collect();

        for (char_i, c) in chars.iter().enumerate() {
            match c {
                '\n' => {
                    i += 1;
                    res.push(vec![]);
                }
                _ => {
                    let r = get_number(&chars, char_i);
                    match r {
                        None => continue,
                        Some(val) => res[i].push(val),
                    }
                }
            }
        }

        return res;
    }
}

lazy_static! {
    static ref TRIE: Trie = {
        Trie::new(vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ])
    };
}

fn get_number(text: &Vec<char>, i: usize) -> Option<u32> {
    let c: char = text[i];
    return match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => c.to_digit(10),
        'o' | 't' | 'f' | 's' | 'e' | 'n' => TRIE.match_at(text, i),
        _ => None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_singleline() {
        let sut = TextNumberProvider {};
        let res = sut.extract_numbers("1abc2");
        assert_eq!(res[0][0], 1);
        assert_eq!(res[0][1], 2);
    }

    #[test]
    fn extract_one_digit() {
        let sut = TextNumberProvider {};
        let res = sut.extract_numbers("7abc");
        assert_eq!(res[0][0], 7);
    }

    #[test]
    fn extract_several_lines() {
        let sut = TextNumberProvider {};
        let res = sut.extract_numbers(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f",
        );
        assert_eq!(res[0][0], 1);
        assert_eq!(res[0][1], 2);
        assert_eq!(res[1][0], 3);
        assert_eq!(res[1][1], 8);
        assert_eq!(res[2][0], 1);
        assert_eq!(res[2][1], 2);
        assert_eq!(res[2][2], 3);
        assert_eq!(res[2][3], 4);
        assert_eq!(res[2][4], 5);
    }

    #[test]
    fn extract_with_text_digit() {
        let sut = TextNumberProvider {};
        let res = sut.extract_numbers(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(res[0][0], 2);
        assert_eq!(res[0][1], 1);
        assert_eq!(res[0][2], 9);

        assert_eq!(res[1][0], 8);
        assert_eq!(res[1][1], 2);
        assert_eq!(res[1][2], 3);

        assert_eq!(res[2][0], 1);
        assert_eq!(res[2][1], 2);
        assert_eq!(res[2][2], 3);

        assert_eq!(res[3][0], 2);
        assert_eq!(res[3][1], 1);
        assert_eq!(res[3][2], 3);
        assert_eq!(res[3][3], 4);

        assert_eq!(res[4][0], 4);
        assert_eq!(res[4][1], 9);
        assert_eq!(res[4][2], 8);
        assert_eq!(res[4][3], 7);
        assert_eq!(res[4][4], 2);

        assert_eq!(res[5][0], 1);
        assert_eq!(res[5][1], 8);
        assert_eq!(res[5][2], 2);
        assert_eq!(res[5][3], 3);
        assert_eq!(res[5][4], 4);

        assert_eq!(res[6][0], 7);
        assert_eq!(res[6][1], 6);
    }
}
