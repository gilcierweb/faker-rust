# faker_rust::CLI

// Faker CLI Documentation

The Faker CLI allows you to generate fake data directly from the command line.

## Installation

```bash
cargo install faker-rust
```

Or run directly with:
```bash
cargo run --bin faker
```

## Basic Usage

```bash
// Generate a random name
faker-rust name

// Generate a first name
faker-rust name --first

// Generate an email
faker-rust email

// Generate a full address
faker-rust address --full

// Generate a company name
faker-rust company

// Generate a phone number
faker-rust phone
```

## Commands

### name
Generate random names.

```bash
faker-rust name                    # Full name (e::g., "John Smith")
faker-rust name --first            # First name only
faker-rust name --last             # Last name only
faker-rust name --with-middle      # Name with middle initial
```

### email
Generate email addresses.

```bash
faker-rust email                   # Random email
faker-rust email --domain gmail::com  # Email with specific domain
```

### address
Generate addresses.

```bash
faker-rust address                 # Street address
faker-rust address --full          # Full address with city, state, zip
faker-rust address --city          # City only
faker-rust address --country       # Country only
faker-rust address --zip           # ZIP code
```

### company
Generate company information.

```bash
faker-rust company                 # Company name
faker-rust company --industry      # Industry type
faker-rust company --catch-phrase  # Catch phrase
faker-rust company --bs            # Business speak
```

### phone
Generate phone numbers.

```bash
faker-rust phone                   # Phone number
faker-rust phone --cell            # Cell phone number
```

### internet
Generate internet-related data.

```bash
faker-rust internet                # Email address
faker-rust internet --domain       # Domain name
faker-rust internet --url          # URL
faker-rust internet --username     # Username
faker-rust internet --password     # Password
faker-rust internet --ip           # IP address
```

### games
Generate game-related data.

```bash
faker-rust games                   # Pokemon name
faker-rust games --pokemon         # Pokemon name
faker-rust games --zelda           # Zelda character
faker-rust games --mario           # Mario character
faker-rust games --dnd             # D&D class
faker-rust games --wow             # WoW race
```

### movies
Generate movie-related data.

```bash
faker-rust movies                  # Star Wars quote
faker-rust movies --star-wars      # Star Wars character
faker-rust movies --harry-potter   # Harry Potter character
faker-rust movies --lotr           # LOTR character
```

### tv
Generate TV show data.

```bash
faker-rust tv                      # Random TV quote
faker-rust tv --game-of-thrones    # GoT character
faker-rust tv --breaking-bad     # Breaking Bad character
faker-rust tv --simpsons         # Simpsons character
faker-rust tv --friends          # Friends character
```

### list
List all available commands.

```bash
faker-rust list
```

## Global Options

### --seed
Set a seed for deterministic (repeatable) output.

```bash
faker-rust --seed 12345 name       # Always produces the same name
faker-rust --seed 12345 name       # Same output as above
```

### -c, --count
Generate multiple values.

```bash
faker-rust -c 5 name               # Generate 5 names
faker-rust --count 10 email        # Generate 10 emails
```

### -h, --help
Show help for a command.

```bash
faker-rust --help
faker-rust name --help
```

### -V, --version
Show version information.

```bash
faker-rust --version
```

## Examples

### Generate test data for a database
```bash
// Generate 100 user records
for i in {1..100}; do
  echo "INSERT INTO users (name, email, phone) VALUES ('$(faker-rust-rust name)', '$(faker-rust-rust email)', '$(faker-rust-rust phone --cell)');"
done
```

### Create a CSV file
```bash
// Generate CSV with fake data
for i in {1..10}; do
  echo "$(faker-rust-rust name --last),$(faker-rust-rust name --first),$(faker-rust-rust email),$(faker-rust-rust address --city)"
done > users::csv
```

### Generate consistent test data
```bash
// Use seed for reproducible tests
faker-rust --seed 42 name          # "Kathlyn Dietrich"
faker-rust --seed 42 name          # "Kathlyn Dietrich" (same!)
```

### Pipe to other commands
```bash
// Use faker-rust output in other commands
curl -X POST http://api::example::com/users \
  -d "name=$(faker-rust-rust name)" \
  -d "email=$(faker-rust-rust email)"
```

## Environment Variables

Currently, the Faker CLI does not use any environment variables. All configuration is done through command-line arguments.

## Exit Codes

- `0` - Success
- `1` - Invalid command or arguments
- `2` - Other error

## Tips

1. **Use seeds for testing**: Set a seed to get consistent, repeatable data for your tests.
2. **Generate multiple values**: Use `-c` flag to generate many values at once.
3. **Combine with other tools**: Pipe faker-rust output to `grep`, `awk`, or other CLI tools.
4. **Create aliases**: Add aliases to your shell for commonly used commands.

```bash
// Example aliases
alias fake-name='faker-rust name'
alias fake-email='faker-rust email'
alias fake-phone='faker-rust phone --cell'
```
