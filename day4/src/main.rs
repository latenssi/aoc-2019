fn main() {
    let input_start = 353_096;
    let input_end = 843_212;

    println!(
        "Different passwords (part1): {}",
        crack(input_start, input_end, -1)
    );
    println!(
        "Different passwords (part2): {}",
        crack(input_start, input_end, 2)
    );
}

fn crack(start: u32, end: u32, max_grp_size: i32) -> u32 {
    (start..end + 1)
        .map(|pass| {
            if validate(pass, start, end, max_grp_size) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn validate(pass: u32, start: u32, end: u32, max_grp_size: i32) -> bool {
    let pass_str = pass.to_string();

    let v_length = pass_str.len() == 6;
    let v_range = start <= pass && pass <= end;
    let mut v_incr = true;
    let mut v_two_adj = false;
    let mut v_grp_size = false;

    let mut grp_size = 1;

    for (i, c) in pass_str.chars().enumerate() {
        if i > 0 {
            let l_d = (pass_str.chars().nth(i - 1).unwrap()).to_digit(10).unwrap();
            let c_d = c.to_digit(10).unwrap();

            v_incr = v_incr && l_d <= c_d;

            if !v_incr {
                break;
            }

            if l_d == c_d {
                grp_size += 1;
            } else {
                grp_size = 1;
            }

            v_two_adj = v_two_adj || grp_size == 2;

            v_grp_size = max_grp_size < 0
                || (2 <= grp_size && grp_size <= max_grp_size)
                || (v_grp_size && grp_size < 2);
        }
    }

    v_length && v_range && v_incr && v_two_adj && v_grp_size
}

#[test]
fn test_validate1() {
    assert_eq!(validate(111_111, 111_111, 111_111, -1), true);
    assert_eq!(validate(223_450, 223_450, 223_450, -1), false);
    assert_eq!(validate(123_789, 123_789, 123_789, -1), false);
}

#[test]
fn test_validate2() {
    assert_eq!(validate(112_233, 112_233, 112_233, 2), true);
    assert_eq!(validate(112_334, 112_334, 112_334, 2), true);
    assert_eq!(validate(111_122, 111_122, 111_122, 2), true);
    assert_eq!(validate(112_333, 112_333, 112_333, 2), false);
    assert_eq!(validate(123_789, 123_789, 123_789, 2), false);
    assert_eq!(validate(123_444, 123_444, 123_444, 2), false);
    assert_eq!(validate(112_223, 112_223, 112_223, 2), false);
}

#[test]
fn test_part1() {
    assert_eq!(crack(111_111, 111_112, -1), 2);
    assert_eq!(crack(353_096, 843_212, -1), 579);
}

#[test]
fn test_part2() {
    assert_eq!(crack(112_333, 112_334, 2), 1);
    assert!(crack(353_096, 843_212, 2) > 274);
}
