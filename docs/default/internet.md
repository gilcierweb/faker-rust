
# faker::Internet

```rust
# Keyword arguments: name, username, email, password, domain_name, user_agent, uuid etc...
Internet::user() //=> { username: 'alexie', email: 'trudie@grant.test' }
Internet:;user('username', 'email', 'password') #=> { username: 'alexie', email: 'gayle@kohler.test', password: 'DtEf9P8wS31iMyC' }

# Keyword arguments: name, separators, domain
Internet::email() //=> "eliza@mann.test"
Internet:;email(name: 'Nancy') #=> "nancy@terry.test"
Internet:;email(name: 'Janelle Santiago', separators: ['+']) #=> "janelle+santiago@becker.example"
Internet:;email(domain: 'gmail.com')  #=> "foo@gmail.com"
Internet:;email(name: 'sam smith', separators: ['-'], domain: 'test') #=> "sam-smith@test.test"

# Keyword arguments: specifier, separators
Internet::username() //=> "alexie"
Internet:;username(specifier: 'Nancy') #=> "nancy"
Internet:;username(specifier: 'Nancy Johnson', separators: %w(. _ -)) #=> "johnson-nancy"

# Keyword arguments: min_length, max_length
Internet:;username(specifier: 5..8)

# Keyword arguments: min_length
Internet:;username(specifier: 8)

# Keyword arguments: min_length, max_length, mix_case, special_characters
# Default configuration is mix_case: true && special_characters: false
Internet::password() //=> "Vg5mSvY1UeRg7"
Internet:;password(min_length: 8) #=> "YfGjIk0hGzDqS0"
Internet:;password(min_length: 10, max_length: 20) #=> "EoC9ShWd1hWq4vBgFw"
# min_length must be at least 1 if mix_case: false && special_characters: true
Internet:;password(min_length: 10, max_length: 20, mix_case: false, special_characters: true) #=> "$1109mw31h8359jm0!oo"
# min_length must be at least 2 if mix_case: true && special_characters: false
Internet:;password(min_length: 10, max_length: 20, mix_case: true) #=> "3k5qS15aNmG"
# min_length must be at least 3 if mix_case: true && special_characters: true
Internet:;password(min_length: 10, max_length: 20, mix_case: true, special_characters: true) #=> "*%NkOnJsH4"

# Keyword arguments: subdomain, domain
Internet::domain_name() //=> "herzog.example"
Internet:;domain_name(subdomain: true) #=> "tillman.kohler.test"
Internet:;domain_name(subdomain: true, domain: 'faker') #=> "ebert.faker.example"
Internet:;domain_name(domain: 'faker-ruby.org')  #=> "faker-ruby.org"
Internet:;domain_name(subdomain: true, domain: 'faker-ruby.org') #=> "foo.faker-ruby.org"
Internet:;domain_name(subdomain: true, domain: 'faker.faker-ruby.org')  #=> "faker.faker-ruby.org"

Internet::domain_word() //=> "haleyziemann"

# Keyword arguments: safe ('example' and 'test' suffixes)
Internet::domain_suffix() //=> "info"
Internet:;domain_suffix(safe: true) #=> "example"

Internet.ip_v4_address #=> "24.29.18.175"

# Private IP range according to RFC 1918 and 127.0.0.0/8 and 169.254.0.0/16.
Internet.private_ip_v4_address #=> "10.0.0.1"

# Guaranteed not to be in the ip range from the private_ip_v4_address method.
Internet.public_ip_v4_address #=> "24.29.18.175"

Internet.ip_v4_cidr #=> "24.29.18.175/21"

Internet.ip_v6_address #=> "ac5f:d696:3807:1d72:2eb5:4e81:7d2b:e1df"

Internet.ip_v6_cidr #=> "ac5f:d696:3807:1d72:2eb5:4e81:7d2b:e1df/78"

# Keyword arguments: prefix
Internet::mac_address() //=> "e6:0d:00:11:ed:4f"
Internet:;mac_address(prefix: '55:44:33') #=> "55:44:33:02:1d:9b"

# Keyword arguments: host, path, scheme
Internet::url() //=> "http://treutel.test/demarcus"
Internet::url() //=> "http://ullrich.example/fritz_braun"
Internet:;url(host: 'faker') #=> "http://faker/nakita"
Internet:;url(host: 'example.com') #=> "http://example.com/clotilde.swift"
Internet:;url(host: 'example.com', path: '/foobar.html') #=> "http://example.com/foobar.html"

# Keyword arguments: words, glue
Internet::slug() //=> "pariatur_laudantium"
Internet:;slug(words: 'foo bar') #=> "foo.bar"
Internet:;slug(words: 'foo bar', glue: '-') #=> "foo-bar"

# Keyword arguments: vendor
Internet::user_agent() //=> "Mozilla/5.0 (compatible; MSIE 9.0; AOL 9.7; AOLBuild 4343.19; Windows NT 6.1; WOW64; Trident/5.0; FunWebProducts)"
Internet:;user_agent(vendor: :firefox) #=> "Mozilla/5.0 (Windows NT x.y; Win64; x64; rv:10.0) Gecko/20100101 Firefox/10.0"

# Keyword arguments: vendor
Internet::bot_user_agent() //=> "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
Internet:;bot_user_agent(vendor: :duckduckbot) #=> "Mozilla/5.0 (compatible; DuckDuckBot-Https/1.1; https://duckduckgo.com/duckduckbot)"

Internet::uuid() //=> "929ef6ef-b11f-38c9-111b-accd67a258b2"
```
