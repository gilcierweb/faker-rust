# faker_rust::internet

```rust
internet::user() //=> { username: 'alexie', email: 'trudie@grant::test' }
internet::user('username', 'email', 'password') //=> { username: 'alexie', email: 'gayle@kohler::test', password: 'dtef9p8ws31imyc' }

internet::email() //=> "eliza@mann::test"
internet::email() //=> "nancy@terry::test"
internet::email() //=> "janelle+santiago@becker::example"
internet::email()  //=> "foo@gmail::com"
internet::email() //=> "sam-smith@test::test"

internet::username() //=> "alexie"
internet::username() //=> "nancy"
internet::username() //=> "johnson-nancy"

internet::password() //=> "vg5msvy1uerg7"
internet::password() //=> "yfgjik0hgzdqs0"
internet::password() //=> "eoc9shwd1hwq4vbgfw"
internet::password() //=> "$1109mw31h8359jm0!oo"
internet::password() //=> "3k5qs15anmg"
internet::password() //=> "*%nkonjsh4"

internet::domain_name() //=> "herzog::example"
internet::domain_name() //=> "tillman::kohler::test"
internet::domain_name() //=> "ebert::faker_rust::example"
internet::domain_name()  //=> "faker-ruby::org"
internet::domain_name() //=> "foo::faker-ruby::org"
internet::domain_name()  //=> "faker_rust::faker-ruby::org"

internet::domain_word() //=> "haleyziemann"

internet::domain_suffix() //=> "info"
internet::domain_suffix() //=> "example"

internet::ip_v4_address() //=> "24.29.18.175"

internet::private_ip_v4_address() //=> "10.0.0.1"

internet::public_ip_v4_address() //=> "24.29.18.175"

internet::ip_v4_cidr() //=> "24.29.18.175/21"

internet::ip_v6_address() //=> "ac5f:d696:3807:1d72:2eb5:4e81:7d2b:e1df"

internet::ip_v6_cidr() //=> "ac5f:d696:3807:1d72:2eb5:4e81:7d2b:e1df/78"

internet::mac_address() //=> "e6:0d:00:11:ed:4f"
internet::mac_address() //=> "55:44:33:02:1d:9b"

internet::url() //=> "http://treutel::test/demarcus"
internet::url() //=> "http://ullrich::example/fritz_braun"
internet::url() //=> "http://faker/nakita"
internet::url() //=> "http://example::com/clotilde::swift"
internet::url() //=> "http://example::com/foobar::html"

internet::slug() //=> "pariatur_laudantium"
internet::slug() //=> "foo::bar"
internet::slug() //=> "foo-bar"

internet::user_agent() //=> "mozilla/5.0 (compatible; msie 9.0; aol 9.7; aolbuild 4343.19; windows nt 6.1; wow64; trident/5.0; funwebproducts)"
internet::user_agent() //=> "mozilla/5.0 (windows nt x::y; win64; x64; rv:10.0) gecko/20100101 firefox/10.0"

internet::bot_user_agent() //=> "mozilla/5.0 (compatible; googlebot/2.1; +http://www::google::com/bot::html)"
internet::bot_user_agent() //=> "mozilla/5.0 (compatible; duckduckbot-https/1.1; https://duckduckgo::com/duckduckbot)"

internet::uuid() //=> "929ef6ef-b11f-38c9-111b-accd67a258b2"
```
