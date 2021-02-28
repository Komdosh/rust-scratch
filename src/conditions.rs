use std::io::stdin;

pub(crate) fn if_statement() {
    let a = 0;
    if i32::pow(a, 0) == 1 {
        println!("it is true");
    } else if a == 5 {
        println!("shouldn't be called");
    }
    let some_value = if a == 0 { "ok" } else { "not ok" };
    println!("how is going? - {}", some_value);
}

pub(crate) fn country_matcher() {
    let country_code = 46;

    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country)
}

pub(crate) fn locked_state() {
    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    println!("enter a code: ");
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED!");
                break;
            }
        }
    }
}

pub(crate) fn options() {
    let x = 3.0;
    let y = 2.0;

    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {
        println!("result = {}", z)
    }
}
