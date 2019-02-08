mod lifetimes {
    pub fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

mod traits {
    //exercises from https://doc.rust-lang.org/stable/book/ch10-02-traits.html

    //restricted to only types that implement `Copy` , i.e. are allocated on the stack
    //because we force a move to largest otherwise
    pub fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    //works for all types but is potentially expensive
    pub fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();

        for item in list.iter() {
            if item > &largest {
                largest = item.clone();
            }
        }

        largest
    }

    //works for all types because it uses immutable references and returns an immutable reference to &T
    pub fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if &item > &largest {
                largest = &item;
            }
        }
        &largest
    }
}

pub fn trait_ex() {
    println!(
        "examples from chapter 10 [ https://doc.rust-lang.org/stable/book/ch10-02-traits.html ]"
    );
    println!("using scalars that implement Copy");
    let l_int = vec![1, 44, 5, 100, 53];
    let l_char = vec!['a', 'b', 't', 'A', 'Z'];

    println!("largest value in l_int is {}", traits::largest_copy(&l_int));
    println!(
        "largest value in l_char is {}",
        traits::largest_copy(&l_char)
    );

    println!("using Clone");
    let l_string = vec![
        String::from("barmmmm"),
        String::from("1foom"),
        String::from("1barm"),
        String::from("fooom"),
    ];
    //reminder, passing an immutable reference &l_string here so that I borrow and don't invalidate l_string or force a return+reassign
    println!(
        "largest value in l_string is {}",
        traits::largest_clone(&l_string)
    );

    println!("using references works for all types");
    //could also dereference (*) each of the following, but Rust does this for me under the covers
    println!("largest value in l_int is {}", traits::largest_ref(&l_int));
    println!(
        "largest value in l_char is {}",
        traits::largest_ref(&l_char)
    );
    println!(
        "largest value in l_string is {}",
        traits::largest_ref(&l_string)
    );
}

pub fn lifetime_ex() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = lifetimes::longest(&string1, string2);
    println!("The longest string is {}", result);
}
