// option and result
// used when an operation might not succeed
// option: Some(value) or None
// result: Ok(value) or Err(error)

#[allow(unused_variables)]

pub fn run() {
    let nothing: Option<i32> = None;
    let maybe_string = Some("hello");

    // nothing.unwrap(); // ! runtime error ("panic")
    let surely_a_string = maybe_string.unwrap_or_default();

    let strings = vec!["1", "2", "3"];
    let nums = parse_numbers(strings);
    println!("{:?}", nums);
}

fn parse_numbers(strings: Vec<&str>) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut nums = Vec::new();

    for s in strings {
        let parsed = s.parse::<i32>();
        if parsed.is_ok() {
            nums.push(parsed.unwrap());
        } else {
            return Err(parsed.unwrap_err());
        }

        match s.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(e) => return Err(e),
        }

        nums.push(s.parse::<i32>()?);
    }

    Ok(nums)
}
