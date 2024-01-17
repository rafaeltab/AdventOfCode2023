pub fn find_valid_numbers(text: &str) -> Vec<u32> {
    let lines: Vec<Vec<char>> = text.lines().map(|x| x.chars().collect()).collect();
    let mut res: Vec<u32> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let line_before = if i == 0 { None } else { Some(&lines[i - 1]) };
        let line_after = if i == lines.len() - 1 {
            None
        } else {
            Some(&lines[i + 1])
        };

        res.extend(find_valid_numbers_on_line(line, line_before, line_after));
    }
    return res;
}

fn find_valid_numbers_on_line(
    line: &Vec<char>,
    before: Option<&Vec<char>>,
    after: Option<&Vec<char>>,
) -> Vec<u32> {
    let number_iterator = NumberIter::new(line);
    return number_iterator
        .filter(|x| x.is_valid(line, before, after))
        .map(|x| x.nr)
        .collect();
}

struct NumberIter<'a> {
    current: usize,
    str: &'a Vec<char>,
}

impl<'a> NumberIter<'a> {
    fn new(str: &'a Vec<char>) -> NumberIter<'a> {
        return NumberIter { current: 0, str };
    }
}

impl<'a> Iterator for NumberIter<'a> {
    type Item = FoundNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut number: Vec<char> = vec![];
        let mut start_pos: Option<usize> = None;

        for i in self.current..self.str.len() {
            if self.str[i].is_ascii_digit() {
                if start_pos.is_none() {
                    start_pos = Some(i);
                }
                number.push(self.str[i]);
            } else {
                if start_pos.is_some() {
                    break;
                }
            }
        }

        if start_pos.is_none() {
            return None;
        }

        self.current = start_pos.unwrap() + number.len();

        return Some(FoundNumber {
            nr: number
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap(),
            start_index: start_pos.unwrap(),
            end_index: (start_pos.unwrap() + number.len() - 1),
        });
    }
}

#[derive(PartialEq, Debug)]
struct FoundNumber {
    nr: u32,
    /// Inclusive
    start_index: usize,
    /// Inclusive
    end_index: usize,
}

impl FoundNumber {
    fn is_valid(
        &self,
        line: &Vec<char>,
        before: Option<&Vec<char>>,
        after: Option<&Vec<char>>,
    ) -> bool {
        let mut min_index = self.start_index;
        let mut max_index = self.end_index;
        if self.start_index != 0 {
            min_index -= 1;
            let line_before = line[self.start_index - 1];

            if line_before != '.' {
                return true;
            }
        }

        if self.end_index != line.len() - 1 {
            max_index += 1;
            let line_after = line[self.end_index + 1];

            if line_after != '.' {
                return true;
            }
        }

        if before.is_some() {
            let before_some = before.unwrap();
            for i in min_index..max_index + 1 {
                let before_char = before_some[i];

                if !before_char.is_digit(10) && before_char != '.' {
                    return true;
                }
            }
        }
        if after.is_some() {
            let after_some = after.unwrap();
            for i in min_index..max_index + 1 {
                let after_char = after_some[i];

                if !after_char.is_digit(10) && after_char != '.' {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_valid_numbers_finds_all_numbers() {
        let challenge = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = find_valid_numbers(challenge);
        assert_eq!(
            res,
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }

    #[test]
    fn find_numbers_on_line_finds_2() {
        let line_b = &"...+......".chars().collect();
        let line_a = &".565.114+.".chars().collect();
        let line_c = &"..........".chars().collect();
        let res = find_valid_numbers_on_line(line_a, Some(line_b), Some(line_c));
        assert_eq!(
            res,
            vec![565, 114]
        );
    }

    #[test]
    fn is_valid() {
        let line_b = &"..........".chars().collect();
        let line_a = &".....114+.".chars().collect();
        let line_c = &"..........".chars().collect();
        let found_number = FoundNumber {
            nr: 114,
            start_index: 5,
            end_index: 7,
        };
        let res = found_number.is_valid(line_a, Some(line_b), Some(line_c));
        assert_eq!(
            res,
            true
        );
    }

    #[test]
    fn is_valid_misses_invalid() {
        let line_b = &"..........".chars().collect();
        let line_a = &".....114.+".chars().collect();
        let line_c = &"..........".chars().collect();
        let found_number = FoundNumber {
            nr: 114,
            start_index: 5,
            end_index: 7,
        };
        let res = found_number.is_valid(line_a, Some(line_b), Some(line_c));
        assert_eq!(
            res,
            false
        );
    }

    #[test]
    fn find_next_number_found() {
        let mut iterator = NumberIter {
            current: 4,
            str: &"467..114..".chars().collect(),
        };
        let res = iterator.next();
        assert_eq!(
            res,
            Some(FoundNumber {
                nr: 114,
                start_index: 5,
                end_index: 7
            })
        );
    }

    #[test]
    fn find_next_number_notfound() {
        let mut iterator = NumberIter {
            current: 8,
            str: &"467..114..".chars().collect(),
        };
        let res = iterator.next();
        assert_eq!(res, None);
    }
}
