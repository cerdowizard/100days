use std::io;
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User{
    fn create_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
    fn get_user(&self){
        println!("{:?}, {:?}, {:?}, {:?}", self.username, self.email, self.sign_in_count, self.active)
    }

    fn update_user(&self, username: String, email: String) -> User {
        User {
            username,
            email: email,
            sign_in_count: self.sign_in_count,
            active: self.active,
        }
    }

    fn deactivate_email(&mut self) -> bool{
        self.active = false;
        self.active
    }
}

fn main() {
    println!("Enter your name: ");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read line");
    let trimed = user_name.trim();

    println!("Enter your email: ");
    let mut user_email = String::new();
    io::stdin().read_line(&mut user_email).expect("Failed to read line");
    let trimed_email = user_email.trim();

    let user = User::create_user(trimed.to_string(), trimed_email.to_string());
    let user_info = user.get_user();
    let updated_user = user.update_user("dave".to_string(), "dave@gmail.com".to_string());

    println!("{:?}", updated_user);
}