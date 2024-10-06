use std::io;

#[derive(Debug)]
enum AccountType {
    Savings,
    Current,
    Local,
}

// impl AccountType {
//     fn to_string(&self) -> String {
//         match self {
//         AccountType::Savings => "savings".to_string(),
//         AccountType::Current => "current".to_string(),
//         AccountType::Local => "local".to_string(),
//         }
//     }
// }
//
#[warn(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    account_type: AccountType,
}

// trait NewTrait {
//     fn create_user(username: String, email: String, account_type:AccountType) -> Self;
//     fn get_user(&self);
//     fn update_user(&self, username: String, email: String, account_type:AccountType) -> User;
//     fn deactivate_email(&mut self) -> bool;
// }
#[allow(dead_code)]
impl User{
    fn create_user(username: String, email: String, account_type:AccountType) -> Self {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
            account_type
        }
    }

    fn get_user(&self){
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", self.username, self.email, self.sign_in_count, self.active, self.account_type)
    }
    fn update_user(&self, username: String, email: String, account_type:AccountType) -> User {
        User {
            username,
            email,
            sign_in_count: self.sign_in_count,
            active: self.active,
            account_type
        }
    }
    fn deactivate_email(&mut self) -> bool{
        self.active = false;
        self.active
    }
}
fn menu_list(){
    println!(" >> Enter 1 to create an account");
    println!(" >> Enter 2 to get account info");
    println!(" >> Enter 3 to activate an account");
    println!(" >> Enter 4 to send money");
    println!(" >> Enter 5 to end application");
}
fn main() {
    loop {
        menu_list();
        let mut user_choice = String::new();
        io::stdin().read_line(&mut user_choice).expect("Failed to read line");

        match user_choice.trim().parse() {
             Ok(1) => {
                println!("Enter your name: ");
                let mut user_name = String::new();
                io::stdin().read_line(&mut user_name).expect("Failed to read line");
                let trimed = user_name.trim();

                println!("Enter your email: ");
                let mut user_email = String::new();
                io::stdin().read_line(&mut user_email).expect("Failed to read line");
                let trimed_email = user_email.trim();

                println!("Enter your account type (savings/current/local): ");
                let mut user_account_type = String::new();
                io::stdin().read_line(&mut user_account_type).expect("Failed to read user input");

                let account_type = match user_account_type.trim() {
                    "savings" => AccountType::Savings,
                    "current" => AccountType::Current,
                    "local" => AccountType::Local,
                    _ => {
                        println!("Invalid account type. Defaulting to Current.");
                        AccountType::Current
                    }
                };

                let user = User::create_user(trimed.to_string(), trimed_email.to_string(), account_type);
            }

            Ok(5) => {
                println!("closing application");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        };

        // user.get_user();
    }
}
