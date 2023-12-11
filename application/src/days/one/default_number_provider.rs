use super::number_provider::NumberProvider;

pub struct DefaultNumberProvider {}

impl NumberProvider for DefaultNumberProvider {
    fn extract_numbers(&self, text: &str) -> Vec<Vec<u32>> {
        let mut i = 0;
        let mut res: Vec<Vec<u32>> = vec![vec![]];

        for c in text.chars() {
            match c {
                '\n' => {
                    i += 1;
                    res.push(vec![]);
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8'| '9' => {
                    res[i].push(c.to_digit(10).expect(""));
                },
                _ => continue
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::days::one::default_number_provider::DefaultNumberProvider;

    use super::*;

    #[test]
    fn extract_singleline() {
        let sut = DefaultNumberProvider{};
        let res = sut.extract_numbers("1abc2");
        assert_eq!(res[0][0], 1);
        assert_eq!(res[0][1], 2);
    }

    #[test]
    fn extract_one_digit() {
        let sut = DefaultNumberProvider{};
        let res = sut.extract_numbers("7abc");
        assert_eq!(res[0][0], 7);
    }

    #[test]
    fn extract_several_lines() {
        let sut = DefaultNumberProvider{};
        let res = sut.extract_numbers("1abc2
pqr3stu8vwx
a1b2c3d4e5f");
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
}
