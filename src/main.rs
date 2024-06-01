use std::env;
use rand::Rng;
const ALL_LETTERS_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const ALL_LETTERS_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALL_CHARACTERS: &str = "!@#$%^&*()-_=+[]{};:,.<>?/`~";

struct PasswordConfigurations {
    length: u32,
    use_special_chars: bool,
    use_uppercase_letters: bool,
    use_lowercase_letters: bool,
}

impl PasswordConfigurations {
    fn get_all_chars(&self) -> String {
        let mut all_chars: String = String::new();

        if self.use_lowercase_letters {all_chars.push_str(ALL_LETTERS_LOWERCASE)}
        if self.use_uppercase_letters {all_chars.push_str(ALL_LETTERS_UPPERCASE)}
        if  self.use_special_chars {all_chars.push_str(ALL_CHARACTERS)}

        return all_chars
    }
}

impl PasswordConfigurations {
    fn get_default_config() -> PasswordConfigurations {
        Self {
            length: 8,
            use_special_chars: false,
            use_uppercase_letters: true,
            use_lowercase_letters: true,
        }
    }

    fn get_config(length: u32, use_special_chars: Option<bool>) -> PasswordConfigurations {
        Self {
            length,
            use_special_chars: use_special_chars.unwrap_or(false),
            use_lowercase_letters: true,
            use_uppercase_letters: true,

        }
    }
}

fn main() {
    let mut configuration = PasswordConfigurations::get_default_config();
    let mut amount: u32 = 1;
    let arguments: Vec<String> = env::args().collect();
    for (index ,argument) in arguments.iter().enumerate() {

        if argument.eq("-s") {
            configuration.use_special_chars = true;
        }

        if argument.eq("--length") || argument.eq("--l") {
            let default_value: String = String::from("8");
            //If the amount is not a number, a default value gets used
            configuration.length = arguments.get(index+1).unwrap_or(&default_value).parse().unwrap_or(8);
        }

        if argument.eq("--amount") || argument.eq("--a"){
            let default_value: String = String::from("1");
            //If the amount is not a number, a default value gets used
            amount = arguments.get(index+1).unwrap_or(&default_value).parse().unwrap_or(1);
        }

        if argument.eq("--help") || argument.eq("-h") || argument.eq("--h") {
            println!("This command generates a random password.
    -s              |   This flag enables the use of special characters
    --amount <n>    |   This is the number on how many passwords get generated (default 1)
    --length <n>    |   This decides the length of the password (default 8)
    -h              |   This help command shows all arguments you can use");
            return;
        }

    }
    if amount <= 1 {
        println!("The generated password is:", );
    } else {
        println!("The generated passwords are:", );
    }

    for _ in 0..amount {
        println!("{}", random_string(&configuration));
    }


}

fn random_string(configuration: &PasswordConfigurations) -> String {
    let mut random_string = String::new();
    let chars = configuration.get_all_chars();
    for _ in 0..configuration.length {
        random_string.push(chars.chars().nth(rand::thread_rng().gen_range(0..chars.len())).unwrap());
    }
    return random_string;
}
