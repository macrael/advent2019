fn valid_password_a(password: String) -> bool {
    let mut prev_char: Option<char> = None;

    let mut seen_double = false;
    for c in password.chars() {
        // If this isn't the first loop.
        if let Some(prev) = prev_char {
            // there's at least 2 characters the same one after another
            if c == prev {
                seen_double = true;
            }

            // all numbers should be strictly increasing
            if c < prev {
                return false;
            }
        }

        prev_char = Some(c);
    }

    return seen_double;
}

fn valid_password_b(password: String) -> bool {
    let mut prev_run = Vec::<char>::new();

    let mut seen_double = false;
    for c in password.chars() {
        // If this isn't the first loop.
        println!("we: {:?}", prev_run);

        if prev_run.len() != 0 {
            let run_c = prev_run[0];
            if c != run_c {
                if prev_run.len() == 2 {
                    seen_double = true;
                }
                // reset our run tracker when the character switches
                prev_run = Vec::new();

                // check that numbers are strictly increasing
                if c < run_c {
                    return false;
                }
            }
        }

        prev_run.push(c);
    }
    // gotta check if the last two were our run
    if prev_run.len() == 2 {
        seen_double = true;
    }

    return seen_double;
}

fn generate_inputs() -> Vec<String> {
    let mut inputs = Vec::new();
    for i in 353096..=843212 {
        inputs.push(i.to_string());
    }

    return inputs;
}

pub fn four_a() -> i32 {
    return generate_inputs().iter().fold(0, |acc, input| {
        // I don't yet understand why I have to to_string here.
        if valid_password_a(input.to_string()) {
            return acc + 1;
        } else {
            return acc;
        }
    });
}

pub fn four_b() -> i32 {
    return generate_inputs().iter().fold(0, |acc, input| {
        // I don't yet understand why I have to to_string here.
        if valid_password_b(input.to_string()) {
            return acc + 1;
        } else {
            return acc;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(true, valid_password_a(String::from("122345")));
        assert_eq!(true, valid_password_a(String::from("111111")));
        assert_eq!(false, valid_password_a(String::from("223450")));
        assert_eq!(false, valid_password_a(String::from("123789")));
        assert_eq!(true, valid_password_a(String::from("111123")));
        assert_eq!(false, valid_password_a(String::from("135679")));
        assert_eq!(true, valid_password_a(String::from("123444")));
    }

    #[test]
    fn two() {
        assert_eq!(true, valid_password_b(String::from("122345")));
        assert_eq!(false, valid_password_b(String::from("111111")));
        assert_eq!(false, valid_password_b(String::from("223450")));
        assert_eq!(false, valid_password_b(String::from("123789")));
        assert_eq!(false, valid_password_b(String::from("111123")));
        assert_eq!(false, valid_password_b(String::from("135679")));
        assert_eq!(true, valid_password_b(String::from("112233")));
        assert_eq!(false, valid_password_b(String::from("123444")));
        assert_eq!(true, valid_password_b(String::from("111122")));
    }
}
