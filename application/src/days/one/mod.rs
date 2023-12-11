pub mod default_number_provider;
pub mod text_number_provider;
pub mod number_provider;

pub fn calibrate(text: &str, number_provider: &dyn number_provider::NumberProvider) -> Vec<u32> {
    let mut res = vec![];
    let numbers = number_provider.extract_numbers(&text);

    for line in numbers {
        if line.len() == 0 {
            continue;
        }
        let first_digit = line[0];
        let last_digit = line[line.len() - 1];

        res.push(first_digit * 10 + last_digit);
    }

    return res;
}

#[cfg(test)]
mod tests {
    use crate::days::one::default_number_provider::DefaultNumberProvider;

    use super::*;

    #[test]
    fn calibrate_singleline() {
        let res = calibrate("1abc2", &DefaultNumberProvider {});
        assert_eq!(res[0], 12);
    }

    #[test]
    fn calibrate_one_digit() {
        let res = calibrate("7abc", &DefaultNumberProvider {});
        assert_eq!(res[0], 77);
    }

    #[test]
    fn calibrate_several_lines() {
        let res = calibrate(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f",
            &DefaultNumberProvider {},
        );
        assert_eq!(res[0], 12);
        assert_eq!(res[1], 38);
        assert_eq!(res[2], 15);
    }
}
