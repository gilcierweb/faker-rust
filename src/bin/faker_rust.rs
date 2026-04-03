//! Faker CLI - Command Line Interface for generating fake data
//!
//! Usage: faker [COMMAND] [OPTIONS]
//!
//! Examples:
//!   faker name                    # Generate a random name
//!   faker name --first            # Generate a random first name
//!   faker email                   # Generate a random email
//!   faker address --full          # Generate a full address
//!   faker company                 # Generate a company name
//!   faker phone                   # Generate a phone number
//!   faker --seed 12345 name       # Generate deterministic output
//!   faker list                    # List all available generators

use clap::{Parser, Subcommand};
use faker_rust::{
    address, company, internet, name, phone_number, Faker,
    games, movies, tv_shows,
};

#[derive(Parser)]
#[command(name = "faker")]
#[command(about = "A CLI tool for generating fake data")]
#[command(version = "0.1.0")]
struct Cli {
    /// Set seed for deterministic output
    #[arg(long, global = true)]
    seed: Option<u64>,

    /// Number of values to generate
    #[arg(short, long, default_value = "1")]
    count: usize,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate names (full, first, last)
    Name {
        #[arg(long)]
        first: bool,
        #[arg(long)]
        last: bool,
        #[arg(long)]
        with_middle: bool,
    },
    /// Generate email addresses
    Email {
        #[arg(long)]
        domain: Option<String>,
    },
    /// Generate addresses
    Address {
        #[arg(long)]
        full: bool,
        #[arg(long)]
        street: bool,
        #[arg(long)]
        city: bool,
        #[arg(long)]
        country: bool,
        #[arg(long)]
        zip: bool,
    },
    /// Generate company names
    Company {
        #[arg(long)]
        industry: bool,
        #[arg(long)]
        catch_phrase: bool,
        #[arg(long)]
        bs: bool,
    },
    /// Generate phone numbers
    Phone {
        #[arg(long)]
        cell: bool,
    },
    /// Generate internet-related data
    Internet {
        #[arg(long)]
        domain: bool,
        #[arg(long)]
        url: bool,
        #[arg(long)]
        username: bool,
        #[arg(long)]
        password: bool,
        #[arg(long)]
        ip: bool,
    },
    /// Generate game-related data
    Games {
        #[arg(long)]
        pokemon: bool,
        #[arg(long)]
        zelda: bool,
        #[arg(long)]
        mario: bool,
        #[arg(long)]
        dnd: bool,
        #[arg(long)]
        wow: bool,
    },
    /// Generate movie-related data
    Movies {
        #[arg(long)]
        star_wars: bool,
        #[arg(long)]
        harry_potter: bool,
        #[arg(long)]
        lotr: bool,
    },
    /// Generate TV show data
    Tv {
        #[arg(long)]
        game_of_thrones: bool,
        #[arg(long)]
        breaking_bad: bool,
        #[arg(long)]
        simpsons: bool,
        #[arg(long)]
        friends: bool,
    },
    /// List all available generators
    List,
}

fn main() {
    let cli = Cli::parse();

    // Set seed if provided
    if let Some(seed) = cli.seed {
        Faker::set_seed(seed);
    }

    for _ in 0..cli.count {
        match &cli.command {
            Commands::Name { first, last, with_middle } => {
                let result = if *first {
                    name::first_name()
                } else if *last {
                    name::last_name()
                } else if *with_middle {
                    name::name_with_middle()
                } else {
                    name::name()
                };
                println!("{}", result);
            }
            Commands::Email { domain } => {
                let domain = domain.as_deref();
                println!("{}", internet::email(None, domain, None));
            }
            Commands::Address { full, street, city: city_only, country: country_only, zip } => {
                let result = if *full {
                    address::full_address()
                } else if *street {
                    address::street_address()
                } else if *city_only {
                    address::city()
                } else if *country_only {
                    address::country()
                } else if *zip {
                    address::zip_code()
                } else {
                    address::street_address()
                };
                println!("{}", result);
            }
            Commands::Company { industry: industry_only, catch_phrase: cp, bs } => {
                let result = if *industry_only {
                    company::industry()
                } else if *cp {
                    company::catch_phrase()
                } else if *bs {
                    company::bs()
                } else {
                    company::name()
                };
                println!("{}", result);
            }
            Commands::Phone { cell } => {
                let result = if *cell {
                    phone_number::cell_phone()
                } else {
                    phone_number::phone_number()
                };
                println!("{}", result);
            }
            Commands::Internet { domain, url, username, password, ip } => {
                let result = if *domain {
                    internet::domain_name(false, None)
                } else if *url {
                    internet::url(None, None, None)
                } else if *username {
                    internet::username(None)
                } else if *password {
                    internet::password(12, 20, true, true)
                } else if *ip {
                    internet::ip_v4()
                } else {
                    internet::email(None, None, None)
                };
                println!("{}", result);
            }
            Commands::Games { pokemon, zelda, mario, dnd, wow } => {
                let result = if *pokemon {
                    games::pokemon::name()
                } else if *zelda {
                    games::zelda::character()
                } else if *mario {
                    games::super_mario::character()
                } else if *dnd {
                    games::dnd::klass()
                } else if *wow {
                    games::world_of_warcraft::race()
                } else {
                    games::pokemon::name()
                };
                println!("{}", result);
            }
            Commands::Movies { star_wars, harry_potter, lotr } => {
                let result = if *star_wars {
                    movies::star_wars::character()
                } else if *harry_potter {
                    movies::harry_potter::character()
                } else if *lotr {
                    movies::lord_of_the_rings::character()
                } else {
                    movies::star_wars::character()
                };
                println!("{}", result);
            }
            Commands::Tv { game_of_thrones, breaking_bad, simpsons, friends } => {
                let result = if *game_of_thrones {
                    tv_shows::game_of_thrones::character()
                } else if *breaking_bad {
                    tv_shows::breaking_bad::character()
                } else if *simpsons {
                    tv_shows::simpsons::character()
                } else if *friends {
                    tv_shows::friends::character()
                } else {
                    tv_shows::rick_and_morty::quote()
                };
                println!("{}", result);
            }
            Commands::List => {
                print_generator_list();
            }
        }
    }
}

fn print_generator_list() {
    println!("Available Generators:");
    println!();
    println!("  NAME:");
    println!("    name              Full name");
    println!("    name --first      First name");
    println!("    name --last       Last name");
    println!("    name --with-middle Name with middle initial");
    println!();
    println!("  EMAIL:");
    println!("    email             Random email");
    println!("    email --domain gmail.com  Email with specific domain");
    println!();
    println!("  ADDRESS:");
    println!("    address           Street address");
    println!("    address --full    Full address");
    println!("    address --city    City name");
    println!("    address --country Country name");
    println!("    address --zip     ZIP code");
    println!();
    println!("  COMPANY:");
    println!("    company           Company name");
    println!("    company --industry Industry type");
    println!("    company --catch-phrase Catch phrase");
    println!();
    println!("  PHONE:");
    println!("    phone             Phone number");
    println!("    phone --cell      Cell phone number");
    println!();
    println!("  INTERNET:");
    println!("    internet          Email address");
    println!("    internet --domain Domain name");
    println!("    internet --url    URL");
    println!("    internet --username Username");
    println!("    internet --password Password");
    println!("    internet --ip     IP address");
    println!();
    println!("  GAMES:");
    println!("    games             Pokemon name");
    println!("    games --pokemon   Pokemon name");
    println!("    games --zelda     Zelda character");
    println!("    games --mario     Mario character");
    println!("    games --dnd       D&D class (klass)");
    println!("    games --wow       WoW race");
    println!();
    println!("  MOVIES:");
    println!("    movies            Star Wars character");
    println!("    movies --star-wars Star Wars character");
    println!("    movies --harry-potter Harry Potter character");
    println!("    movies --lotr     LOTR character");
    println!();
    println!("  TV:");
    println!("    tv                Random TV quote");
    println!("    tv --game-of-thrones GoT character");
    println!("    tv --breaking-bad Breaking Bad character");
    println!("    tv --simpsons     Simpsons character");
    println!("    tv --friends      Friends character");
    println!();
    println!("Options:");
    println!("  --seed NUMBER       Set seed for deterministic output");
    println!("  -c, --count NUMBER  Number of values to generate");
    println!("  -h, --help          Print help");
    println!("  -V, --version       Print version");
}
