use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    for arg_str in args {
        let string: String = if is_consonant(&arg_str.chars().next().unwrap()) {
            let mut pig = String::from("-");
            pig.push_str(&arg_str[0..1]);
            pig.push_str("ay");
            let mut tmp: String = String::from(&arg_str[1..]);
            tmp.push_str(&pig);
            tmp
        }
        else {
            let mut pig = String::from(&arg_str);
            pig.push_str("-ay");
            pig
        };

        print!("{} ", string);
    }
    println!();
}

fn is_consonant(ch: &char) -> bool {
    match ch {
        'a' => false,
        'e' => false,
        'i' => false,
        'o' => false,
        'u' => false,
        'y' => false,
        _   => {
            //println!("It's a consonant");
            true
        },
    }
}
