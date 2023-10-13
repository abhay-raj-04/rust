extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq)]
pub struct Venue {
    id: u32,
    username: String,
    company_name: String,
    owner_name: String,
    state: String,
    district: String,
    contact_number: String,
    person_capacity:(u32,u32),
    ac_availability:bool,
    wheelchair_availability:bool,
    parking:bool,
    partners: Vec<Partner>,
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct Partner {
    spid:u32,
    name: String,
    services_provided: Vec<String>,
    availability:bool
}

fn main() {
    loop {
        println!("Welcome to the Signup and Login Page!");
        println!("1. Signup");
        println!("2. Login");

        let choice = user_input_int("Enter your choice (1 or 2): ");

        match choice {
            1 => signup(),
            2 => {
                if let Some(user) = login() {
                    user_menu(user);
                } else {
                    println!("Login failed. Invalid username or password.");
                }
            }
            _ => {
                println!("Invalid choice!");
                break;
            }
        }
    }
}

fn signup() {
    let username = user_input_string("Enter your username: ");
    let password = user_input_string("Enter your password: ");
    let mut existing_users = read_user_data().unwrap_or_default();

    let user = User {
        username: username,
        password: password,
    };
    if existing_users.iter().any(|u| u.username == user.username) {
        println!("User with the same username already exists.");
        return;
    }
    existing_users.push(user);
    existing_users.sort_by(|a, b| a.username.cmp(&b.username));

    let user_json =
        serde_json::to_string_pretty(&existing_users).expect("JSON serialization failed");
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("acc.json")
    {
        if file.write_all(user_json.as_bytes()).is_ok() {
            println!("User data saved successfully to acc.json.");
        } else {
            eprintln!("Failed to save user data.");
        }
    } else {
        eprintln!("Failed to create or open acc.json file.");
    }
}

fn login() -> Option<User> {
    let username = user_input_string("Enter your username: ");
    let password = user_input_string("Enter your password: ");

    let existing_users = match read_user_data() {
        Ok(users) => users,
        Err(_) => Vec::new(),
    };

    if let Some(user) = existing_users
        .iter()
        .find(|user| user.username == username && user.password == password)
    {
        println!("Login successful!");
        Some(user.clone())
    } else {
        println!("Login failed. Invalid username or password.");
        None
    }
}

fn user_menu(logged_in_user: User) {
    println!("Welcome, {}!", logged_in_user.username);

    loop {
        println!("1. Venue Registration");
       // println!("2. See Available Venues");
        println!("3. Add Partners to existing venues");
        println!("4. Exit");
        println!("5 book a venue:");
        println!("6  register partner:");

        let choice = user_input_int("Enter your choice (1, 2, or 3): ");

        match choice {
            1 => register::venuehub::venue_registration(&logged_in_user),
            // 2 => see_available_venues(&logged_in_user.username),
            3 =>{register::venuehub::list_user_venues(&logged_in_user); },
            4 => {
                println!("Goodbye!");
                break;
            }
            5 => book::booking::book_venue(),
            6 => service::services_provided::register_services(),
            _ => println!("Invalid choice!"),
        }
    }
}

fn read_user_data() -> Result<Vec<User>, serde_json::Error> {
    let contents = fs::read_to_string("acc.json").unwrap_or_default();
    serde_json::from_str(&contents).map_err(serde_json::Error::from)
}
fn read_venue_data() -> Result<Vec<Venue>, serde_json::Error> {
    let contents = fs::read_to_string("venue.json").unwrap_or_default();
    serde_json::from_str(&contents).map_err(serde_json::Error::from)
}
fn read_servicers_data() -> Result<Vec<Partner>, serde_json::Error> {
    let contents = fs::read_to_string("partner.json").unwrap_or_default();
    serde_json::from_str(&contents).map_err(serde_json::Error::from)
}

fn user_input_bool(message: &str) -> bool {
    println!("{}", message);
    let input = user_input_string("Enter y or n");
    if input == "y" {
        true
    } else {
        false
    }
}
fn user_input_string(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input.trim().to_string()
}
fn user_input_int(message: &str) -> i32 {
    println!("{}", message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input.trim().parse().unwrap()
}
fn new_input() -> String{
let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input.trim().to_string()
}
fn capacity(x: i32) -> (u32, u32) {
    match x {
        1..=99 => (50, 100),     // Capacity range for 1-99 people
        100..=199 => (100, 200), // Capacity range for 100-199 people
        _ => (200, 300),         // Default capacity range for other values
    }
    
}
fn auto_generate_id() -> u32 {
    let mut old_id = 01;
    let existng_venue = read_venue_data().unwrap_or_default();

    loop {
        if existng_venue.iter().any(|id| id.id == old_id) {
            old_id += 1;
        } else {
            break old_id;
        }
    }
}
 use std::collections::HashMap;


    pub fn venue_registration(logged_in_user: &User) {
        let id = auto_generate_id();
        let username = logged_in_user.username.clone();
        let company_name = user_input_string("Enter Venue Company Name: ").to_uppercase();
        let owner_name = user_input_string("Enter Owner Name: ").to_uppercase();
        let state = user_input_string("Enter State: ").to_uppercase();
        let district = user_input_string("Enter District: ").to_uppercase();
        let contact_number = user_input_string("Enter mobile no").parse().unwrap();
        let partners = Vec::new();
        let person_capacity = capacity(user_input_int("Enter approx person capacity"));
        let (ac_availability, wheelchair_availability, parking) = (
            user_input_bool("Ac Services"),
            user_input_bool("Wheel chair services"),
            user_input_bool("Parking Facility"),
        );

        let mut existing_venues = read_venue_data().unwrap_or_default();

        let id = id as u32;

        let venue = Venue {
            id,
            username,
            company_name,
            owner_name,
            state,
            district,
            contact_number,
            person_capacity,
            ac_availability,
            wheelchair_availability,
            parking,
            partners,
        };

        existing_venues.push(venue);

        let venue_json =
            serde_json::to_string_pretty(&existing_venues).expect("JSON serialization failed");

        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("venue.json")
        {
            if file.write_all(venue_json.as_bytes()).is_ok() {
                println!("Venue data saved successfully to venue.json.");
            } else {
                eprintln!("Failed to save venue data.");
            }
        } else {
            eprintln!("Failed to create or open venue.json file.");
        }
    }

    pub fn select_partner(mut venues: Venue) {
        let mut existing_venues = read_venue_data().unwrap_or_default();
        let mut existing_partners = read_servicers_data().unwrap_or_default();

        // Debug print to check if existing_partners contains data
        println!("Available Partners:");
        for partners in &existing_partners {
            println!(
                "Partner name: {} \n Services: {:?}",
                partners.name.clone(),
                partners.services_provided.clone()
            );
        }

        let selection = user_input_int("How many partners do you want to select?");

        for i in 1..=selection {
            println!("Enter the ID of partner {}", i);
            for partners in &existing_partners {
                if partners.spid == i as u32 {
                    venues.partners.push(partners.clone());
                }
            }
        }

        // Update the venue data in existing_venues
        for venue in existing_venues.iter_mut() {
            if venue.id == venues.id {
                // Update the partners for the specific venue
                venue.partners = venues.partners.clone();
            }
        }

        // Serialize the updated venue data
        let venue_json =
            serde_json::to_string_pretty(&existing_venues).expect("JSON serialization failed");

        // Write the updated data to "venue.json"
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("venue.json")
        {
            if file.write_all(venue_json.as_bytes()).is_ok() {
                println!("Venue data updated successfully in venue.json.");
            } else {
                eprintln!("Failed to update venue data.");
            }
        } else {
            eprintln!("Failed to create or open venue.json file.");
        }
    }

    pub fn list_user_venues(logged_in_user: &User) -> Option<Venue> {
    let mut existing_venues = read_venue_data().unwrap_or_default();
    
    // Display available venues for the logged-in user
    println!("Your Venues:");
    for venue in &existing_venues {
        if venue.username == logged_in_user.username {
            println!(
                "ID: {}\nCompany Name: {}\nLocation: {}, {}\n",
                venue.id,
                venue.company_name,
                venue.district,
                venue.state
            );
        }
    }

    let selection = user_input_int("Enter the ID of the venue to add partners (0 to cancel):");

    if selection == 0 {
        None // User canceled the operation
    } else {
        existing_venues.iter()
            .find(|venue| venue.id == selection as u32 && venue.username == logged_in_user.username)
            .cloned() // Return the selected venue or None if not found
    }
}

pub fn add_partners(logged_in_user: &User) {
    if let Some(selected_venue) = list_user_venues(logged_in_user) {
        select_partner(selected_venue);
    } else {
        println!("Operation canceled or invalid selection.");
    }
}
pub fn register_services() {
        let mut existing_partners = read_servicers_data().unwrap_or_default();
        let mut partners = vec![];
        let spid = auto_generate_id();
        let partner_name = user_input_string("Enter partner company name: ");

        println!(
            "Enter the number of services provided by{}: ",
            &partner_name
        );
        let services_count = new_input();
        let mut services_provided: Vec<String> = Vec::new();

        for i in 0..services_count.parse().unwrap() {
            println!("Enter Service {}: ", i);
            let service = new_input();
            services_provided.push(service);
        }
        println!("Is {} avaialable for service?", &partner_name);
        let availability = new_input().parse().unwrap();
        // // Check if the partner with the same name already exists in the venue
        // if let Some(existing_partner) = partner_details.partners.iter_mut().find(|p| p.name == partner_name)
        // {
        //     // Update the existing partner's services
        //     existing_partner.services_provided = services_provided;
        // } else {
        // Add a new partner
        let partner = Partner {
            spid,
            name: partner_name.clone(),
            services_provided,
            availability,
        };

        partners.push(partner.clone());
        existing_partners.push(partner.clone());

        let partner_json =serde_json::to_string_pretty(&existing_partners).expect("JSON serialization failed");

        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("partner.json")
        {
            if file.write_all(partner_json.as_bytes()).is_ok() {
                println!("partner data saved successfully to partner.json.");
            } else {
                eprintln!("Failed to save partner data.");
            }
        } else {
            eprintln!("Failed to create or open partner.json file.");
        }
    }