// functions
// note: no ; at the end of a statement means return
// closures: inline functions
// |arg1, arg2| arg1 + arg2
// functions can be passed as arguments or returned from other functions
// generics: use the same function for different types

pub fn run() {
    greeting("hello", "tom");

    let get_sum = add(5, 5);
    println!("sum: {}", get_sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // access to outside variables
    println!("sum +10: {}", add_nums(3, 3));

    let nums = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for_each(&nums, &mut |n| sum += n); // fn has to be mutable to mutate outside variables
    println!("array sum: {}", sum);

    let mut count = 0;
    {
        let mut c = closure_count(&mut count);
        for _ in 1..3 {
            c();
        }
    }
    println!("closure count: {}", count);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // no ";" means return this value
}

fn for_each<T>(arr: &[T], callback: &mut dyn FnMut(&T)) {
    for item in arr {
        callback(item);
    }
}

fn closure_count<'a>(count: &'a mut i32) -> impl FnMut() + 'a {
    return move || {
        *count += 1;
    };
}
