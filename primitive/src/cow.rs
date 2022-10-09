use std::borrow::Cow;

// The type Cow is a smart pointer providing clone-on-write functionality: 
// it can enclose and provide immutable access to borrowed data, 
// and clone the data lazily when mutation or ownership is required. 
pub fn cow_example() {
    let origin = "hello world";
    let mut cow = Cow::from(origin);
    assert_eq!(cow, "hello world");

    // Cow can be borrowed as a str
    let s: &str = &cow;
    assert_eq!(s, "hello world");

    // Cow can be borrowed as a mut str
    let s: &mut str = cow.to_mut();
    s.make_ascii_uppercase();
    assert_eq!(s, "HELLO WORLD");
    assert_eq!(origin, "hello world");

    // Cow can be cloned
    let cow2 = cow.clone();
    assert_eq!(cow2, "HELLO WORLD");
    assert_eq!(origin, "hello world");

    // Cow can be converted to a String
    let s: String = cow.into();
    assert_eq!(s, "HELLO WORLD");
}

pub fn cow_example2() {
    fn abs_all(input: &mut Cow<[i32]>) {
        for i in 0..input.len() {
            let v = input[i];
            if v < 0 {
                // Clones into a vector if not already owned.
                input.to_mut()[i] = -v;
            }
        }
    }

    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("origin: {:?}, mutated: {:?}", &slice, &input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("origin: {:?}, mutated: {:?}", &slice, &input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
}