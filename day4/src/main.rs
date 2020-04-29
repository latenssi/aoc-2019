fn main() {
    let input_start = 353_096;
    let input_end = 843_212;

    println!("Different passwords: {}", crack(input_start, input_end));
}

fn crack(start: u32, end: u32) -> u32 {
    (start..end+1).map(|pass|
        if validate(pass, start, end) { 1 }
        else { 0 }
    ).sum()
}

fn validate(pass: u32, start: u32, end: u32) -> bool {
    let pass_str = pass.to_string();

    let v_length = pass_str.len() == 6;
    let v_range = start <= pass && pass <= end;
    let mut v_two_adj = false;
    let mut v_incr = true;

    for (i, c) in pass_str.chars().enumerate() {
        if i > 0 {
            let last_c = pass_str.chars().nth(i-1).unwrap();
            v_two_adj = v_two_adj || last_c == c;
            v_incr = v_incr && (last_c as u32) <= (c as u32);
        }
    }

    v_length && v_range && v_two_adj && v_incr
}

#[test]
fn test_part1() {
    assert_eq!(crack(111_111, 111_112), 2);
    assert_eq!(crack(353_096, 843_212), 579);
}
