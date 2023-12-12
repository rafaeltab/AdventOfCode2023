pub fn extract_possible_games(text: &str, max_red: i32, max_blue: i32, max_green: i32) -> Vec<LineInfo> {
    let mut possible_games = vec![];
    for line in text.lines() {
        let line_info = parse_line(line);
        if line_info.is_possible(max_red, max_blue, max_green) {
            possible_games.push(line_info);
        }
    }
    return possible_games;
}

pub struct LineInfo {
    pub nr: i32,
    sets: Vec<SetInfo>,
}

impl LineInfo {
    fn is_possible(&self, max_red: i32, max_blue: i32, max_green: i32) -> bool {
        for set in &self.sets {
            if set.red_count > max_red || set.blue_count > max_blue || set.green_count > max_green
            {
                return false;
            }
        }
        return true;
    }
}

pub struct SetInfo {
    red_count: i32,
    blue_count: i32,
    green_count: i32,
}

fn parse_line(line: &str) -> LineInfo {
    let mut draft_line_info = LineInfo {
        nr: 0,
        sets: vec![],
    };

    let res = line.chars().skip(5).take_while(|x| *x != ':');
    let line_nr_str: String = res.into_iter().collect();

    draft_line_info.nr = line_nr_str.parse().unwrap();
    draft_line_info.sets = line
        .split(": ")
        .skip(1)
        .last()
        .unwrap()
        .split("; ")
        .map(|x| parse_subset(x))
        .collect::<Vec<SetInfo>>();

    return draft_line_info;
}

fn parse_subset(set: &str) -> SetInfo {
    let mut draft_set_info = SetInfo {
        red_count: 0,
        blue_count: 0,
        green_count: 0,
    };
    for color_text in set.split(", ") {
        let mut split = color_text.split(" ");
        let nr: i32 = split.next().unwrap().parse().unwrap();
        let color = split.next().unwrap();
        match color {
            "blue" => draft_set_info.blue_count = nr,
            "red" => draft_set_info.red_count = nr,
            "green" => draft_set_info.green_count = nr,
            _ => panic!("Unexpected color"),
        }
    }

    return draft_set_info;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_single_possible() {
        let res = extract_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 12, 14, 13);
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn extract_single_impossible() {
        let res = extract_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1, 14, 13);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn extract_example() {
        let res = extract_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 12, 14, 13);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].nr, 1);
        assert_eq!(res[1].nr, 2);
        assert_eq!(res[2].nr, 5);
    }
}
