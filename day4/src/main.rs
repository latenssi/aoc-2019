fn main() {
    let input_start = 353_096;
    let input_end = 843_212;

    println!(
        "Different passwords (part1): {}",
        crack(input_start, input_end, false)
    );
    println!(
        "Different passwords (part2): {}",
        crack(input_start, input_end, true)
    );
}

fn crack(start: u32, end: u32, limit_grp_len: bool) -> u32 {
    (start..end + 1)
        .map(|password| {
            if validate(password, start, end, limit_grp_len) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn validate(password: u32, start: u32, end: u32, limit_grp_len: bool) -> bool {
    let pass_str = password.to_string();

    let v_length = pass_str.len() == 6;
    let v_range = start <= password && password <= end;
    let mut v_num_incr = false;
    let mut v_two_adj = false;
    let mut v_grp_len = false;

    let mut prev_grp_len = 1;
    let mut grp_len = 1;

    for (i, c) in pass_str.chars().enumerate() {
        if i == 0 {
            // Skip the first digit (we need a pair of digits to compare)
            continue;
        }

        // Last digit
        let l_d = (pass_str.chars().nth(i - 1).unwrap()).to_digit(10).unwrap();

        // Current digit
        let c_d = c.to_digit(10).unwrap();

        v_num_incr = l_d <= c_d;

        if !v_num_incr {
            // Current digit is decreasing compared to last
            // No need to test rest of the digits => break
            break;
        }

        if l_d == c_d {
            // Adjacent digits match => increment group length counter
            grp_len += 1;
        } else {
            // Adjacent digits don't match => reset group length counter
            if grp_len == 2 {
                // If a group was 2 digits long, record it
                prev_grp_len = grp_len;
            }
            grp_len = 1;
        }

        v_two_adj = grp_len == 2 || v_two_adj;
        v_grp_len = grp_len == 2 || prev_grp_len == 2;
    }

    v_length && v_range && v_num_incr && v_two_adj && (!limit_grp_len || v_grp_len)
}

#[test]
fn test_validate1() {
    assert_eq!(validate(111_111, 111_111, 111_111, false), true);
    assert_eq!(validate(223_450, 223_450, 223_450, false), false);
    assert_eq!(validate(123_789, 123_789, 123_789, false), false);
}

#[test]
fn test_validate2() {
    assert_eq!(validate(112_233, 112_233, 112_233, true), true);
    assert_eq!(validate(112_334, 112_334, 112_334, true), true);
    assert_eq!(validate(111_122, 111_122, 111_122, true), true);
    assert_eq!(validate(111_223, 111_223, 111_223, true), true);
    assert_eq!(validate(122_233, 122_233, 122_233, true), true);
    assert_eq!(validate(111_233, 111_233, 111_233, true), true);
    assert_eq!(validate(112_333, 112_333, 112_333, true), true);
    assert_eq!(validate(112_223, 112_223, 112_223, true), true);
    assert_eq!(validate(111_111, 111_111, 111_111, true), false);
    assert_eq!(validate(122_234, 122_234, 122_234, true), false);
    assert_eq!(validate(123_789, 123_789, 123_789, true), false);
    assert_eq!(validate(123_444, 123_444, 123_444, true), false);
}

#[test]
fn test_part1() {
    assert_eq!(crack(111_111, 111_112, false), 2);
    assert_eq!(crack(353_096, 843_212, false), 579);
}

#[test]
fn test_part2() {
    assert_eq!(crack(112_333, 112_334, true), 2);
    assert!(crack(353_096, 843_212, true) > 274);
}
