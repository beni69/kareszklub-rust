// reference pointer: points to a resource in memory

pub fn run() {
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));
    // works, primitive values copy, don't move

    // dynamically sized array
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2)); // works

    let vec3 = vec2;
    println!("{:?}", (&vec1, vec2, vec3));
    // also works, the reference itself is a primitive value

    // mutable references
    // either infinite immutable or one mutable reference is allowed
    let mut num = 5;
    let num_ref = &mut num;
    *num_ref += 1; //. * -> dereference operator
    assert_eq!(num, 6);
}
