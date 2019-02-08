
#[derive(Debug)]
struct User{
  email:String,
  username:String,
  active:bool,
  sign_in_count: u64
}

fn build_mut_user(email: String, username: String) -> User {
   User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let u1 = build_mut_user("em".to_string(), "uname".to_string());
    /*fails with ```
    error[E0382]: borrow of moved value: `u1`
  --> src/main.rs:22:29
   |
20 |     let u1 = build_mut_user("em".to_string(), "uname".to_string());
   |         -- move occurs because `u1` has type `User`, which does not implement the `Copy` trait
21 |     let u2 = {..u1};
   |                 -- value moved here
22 |     println!("u1 is {:#?}", u1);
   |                             ^^ value borrowed here after move
   ```
    */
    //let u2 = User{..u1};

    let u2 = User{
        email: String::from("em2"),
        username: String::from("uname2"),
        ..u1};

    println!("u1 is {:#?}", u1);
    println!("u2 is {:#?}", u1);
}
