fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointers, borrow the vector mutably
    { //This scope ensures that the mutable reference to v goes out of scope before modifying v.
        let first = &mut v[0];
        *first = 10;    
    }
    println!("The first element is: {}", v[0]);
}