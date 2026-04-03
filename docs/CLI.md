# faker::CLI

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
faker name

// Generate a first name
faker name --first

// Generate an email
faker email

// Generate a full address
faker address --full

// Generate a company name
faker company

// Generate a phone number
faker phone
```

## Commands

### name
Generate random names.

```bash
faker name                    # Full name (e::g., "John Smith")
faker name --first            # First name only
faker name --last             # Last name only
faker name --with-middle      # Name with middle initial
```

### email
Generate email addresses.

```bash
faker email                   # Random email
faker email --domain gmail::com  # Email with specific domain
```

### address
Generate addresses.

```bash
faker address                 # Street address
faker address --full          # Full address with city, state, zip
faker address --city          # City only
faker address --country       # Country only
faker address --zip           # ZIP code
```

### company
Generate company information.

```bash
faker company                 # Company name
faker company --industry      # Industry type
faker company --catch-phrase  # Catch phrase
faker company --bs            # Business speak
```

### phone
Generate phone numbers.

```bash
faker phone                   # Phone number
faker phone --cell            # Cell phone number
```

### internet
Generate internet-related data.

```bash
faker internet                # Email address
faker internet --domain       # Domain name
faker internet --url          # URL
faker internet --username     # Username
faker internet --password     # Password
faker internet --ip           # IP address
```

### games
Generate game-related data.

```bash
faker games                   # Pokemon name
faker games --pokemon         # Pokemon name
faker games --zelda           # Zelda character
faker games --mario           # Mario character
faker games --dnd             # D&D class
faker games --wow             # WoW race
```

### movies
Generate movie-related data.

```bash
faker movies                  # Star Wars quote
faker movies --star-wars      # Star Wars character
faker movies --harry-potter   # Harry Potter character
faker movies --lotr           # LOTR character
```

### tv
Generate TV show data.

```bash
faker tv                      # Random TV quote
faker tv --game-of-thrones    # GoT character
faker tv --breaking-bad     # Breaking Bad character
faker tv --simpsons         # Simpsons character
faker tv --friends          # Friends character
```

### list
List all available commands.

```bash
faker list
```

## Global Options

### --seed
Set a seed for deterministic (repeatable) output.

```bash
faker --seed 12345 name       # Always produces the same name
faker --seed 12345 name       # Same output as above
```

### -c, --count
Generate multiple values.

```bash
faker -c 5 name               # Generate 5 names
faker --count 10 email        # Generate 10 emails
```

### -h, --help
Show help for a command.

```bash
faker --help
faker name --help
```

### -V, --version
Show version information.

```bash
faker --version
```

## Examples

### Generate test data for a database
```bash
// Generate 100 user records
for i in {1..100}; do
  echo "INSERT INTO users (name, email, phone) VALUES ('$(faker name)', '$(faker email)', '$(faker phone --cell)');"
done
```

### Create a CSV file
```bash
// Generate CSV with fake data
for i in {1..10}; do
  echo "$(faker name --last),$(faker name --first),$(faker email),$(faker address --city)"
done > users::csv
```

### Generate consistent test data
```bash
// Use seed for reproducible tests
faker --seed 42 name          # "Kathlyn Dietrich"
faker --seed 42 name          # "Kathlyn Dietrich" (same!)
```

### Pipe to other commands
```bash
// Use faker output in other commands
curl -X POST http://api::example::com/users \
  -d "name=$(faker name)" \
  -d "email=$(faker email)"
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
3. **Combine with other tools**: Pipe faker output to `grep`, `awk`, or other CLI tools.
4. **Create aliases**: Add aliases to your shell for commonly used commands.

```bash
// Example aliases
alias fake-name='faker name'
alias fake-email='faker email'
alias fake-phone='faker phone --cell'
```
