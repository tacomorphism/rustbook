mod trait_ex;

fn main() {
    println!("examples from chapter 10 [ https://doc.rust-lang.org/stable/book/ch10-02-traits.html ]");
    println!("using scalars that implement Copy");
    let l_int = vec![1,44,5,100, 53];
    let l_char = vec!['a', 'b', 't', 'A', 'Z'];

    println!("largest value in l_int is {}", trait_ex::largest_copy(&l_int));
    println!("largest value in l_char is {}", trait_ex::largest_copy(&l_char));

    println!("using Clone");
    let l_string = vec![String::from("barmmmm"), String::from("1foom"), String::from("1barm"), String::from("fooom")];
    //reminder, passing an immutable reference &l_string here so that I borrow and don't invalidate l_string or force a return+reassign
    println!("largest value in l_string is {}", trait_ex::largest_clone(&l_string));

    println!("using references works for all types");
    //could also dereference (*) each of the following, but Rust does this for me under the covers
    println!("largest value in l_int is {}", trait_ex::largest_ref(&l_int));
    println!("largest value in l_char is {}", trait_ex::largest_ref(&l_char));    
    println!("largest value in l_string is {}", trait_ex::largest_ref(&l_string));
}
