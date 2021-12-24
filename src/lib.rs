use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io};

// Constants
const HARD_PITY: u32 = 90;
const SAVE_DATA: &str = "wish_data.json";

#[derive(Debug, Serialize, Deserialize)]
struct WishData {
    number_of_wishes: u32,
    was_prev_featured: Option<bool>,
}

// Functions
fn load_data() -> Result<WishData, Box<dyn Error>> {
    let saved_data = match fs::read_to_string(SAVE_DATA) {
        Ok(s) => serde_json::from_str(&s)?,
        _ => WishData {
            number_of_wishes: 0,
            was_prev_featured: None,
        },
    };
    Ok(saved_data)
}

fn save_data(data: &WishData) -> Result<(), Box<dyn Error>> {
    let serialized_data = serde_json::to_string(&data)?;
    fs::write(SAVE_DATA, serialized_data)?;
    Ok(())
}

fn get_input(msg: &str) -> Result<String, Box<dyn Error>> {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut data = load_data()?;

    loop {
        match data.was_prev_featured {
            Some(false) => println!(
                "{} wishes until FEATURED 5 star",
                HARD_PITY - data.number_of_wishes
            ),
            _ => println!("{} wishes until 5 star", HARD_PITY - data.number_of_wishes),
        }

        let input = get_input("Enter number of wishes spent: ")?;
        if input == "" {
            break;
        }
        data.number_of_wishes += input.parse::<u32>()?;

        if get_input("Five star? (y/N)")? == "y" {
            if get_input("Featured? (y/N)")? == "y" {
                data.was_prev_featured = Some(true);
            } else {
                data.was_prev_featured = Some(false);
            }
            data.number_of_wishes = 0;
        }

        save_data(&data)?;
    }

    Ok(())
}
