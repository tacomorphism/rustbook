//exercises from https://doc.rust-lang.org/stable/book/ch10-02-traits.html
//I'm doing this for practice with moves and borrows, but a more elegant solution would be to 
//track `largest` with a mutable index

//restricted to only types that implement `Copy` , i.e. are allocated on the stack
//because we force a move to largest otherwise
    pub fn largest_copy<T:PartialOrd + Copy>(list: &[T]) -> T{
        let mut largest = list[0];

        for &item in list.iter(){
            if item > largest{
                largest = item;
            }
        }

        largest
    }

//works for all types but is potentially expensive
    pub fn largest_clone<T:PartialOrd + Clone>(list: &[T]) -> T{
        let mut largest = list[0].clone();

        for item in list.iter(){
            if item > &largest{
                largest = item.clone();
            }
        }

        largest
    }

//works for all types because it uses immutable references and returns an immutable reference to &T
    pub fn largest_ref<T:PartialOrd>(list :&[T]) -> &T{
        let mut largest = &list[0];

        for item in list.iter(){
            if &item > &largest{
                largest = &item;
            }
        }
        &largest
    }