const INPUT_START: u32 = 372037;
const INPUT_END: u32 = 905157;

trait FetchDigit {
    fn set_digits(&mut self, vals: &[u32]);
    fn get_digits(&self) -> [u32; 6];
}

impl FetchDigit for u32 {
    fn set_digits(&mut self, vals: &[u32]) {
        let mut new_digit = vals[0];

        for i in 1..6 {
            new_digit *= 10;
            new_digit += vals[i];
        }

        *self = new_digit;
    }

    fn get_digits(&self) -> [u32; 6] {
        let mut val = *self;
        let mut output = [0; 6];
        for i in 0..6 {
            output[5 - i] = val % 10;
            val /= 10;
        }

        output
    }
}

pub fn aoc() {
    let mut i = INPUT_START;
    let mut passwords = 0;
    'pass_loop: while i <= INPUT_END {
        let mut valid = false;
        let mut matches = 1;
        let mut digits = i.get_digits();
        let mut prev_digit = digits[0];

        for j in 1..6 {
            let curr_digit = digits[j as usize];
            if curr_digit < prev_digit {
                for k in j..6 {
                    digits[k as usize] = prev_digit;
                }
                i.set_digits(&digits);
                continue 'pass_loop;
            } else if prev_digit == curr_digit {
                matches += 1;
            } else {
                if matches == 2 {
                    valid = true;
                } else {
                    matches = 1;
                }
            }

            prev_digit = curr_digit;
        }

        if valid || matches == 2 {
            passwords += 1;
        }

        i += 1;
    }

    println!("{}", passwords);
}
