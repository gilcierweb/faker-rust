# faker_rust::internet

```rust
internet::user(vec!["username", "email"]) //=> {"username": "oscar.smitham", "email": "clair.sipes@info"}

internet::email(None, None, None) //=> "chante.homenick@io"

internet::username(None) //=> "sanjuana.schaefer"

internet::password(8, 16, true, true) //=> "&7ZtVB#NfN"

internet::domain_name(false, None) //=> "dickinson,.info"

internet::domain_suffix(false) //=> "biz"

internet::fix_umlauts("Jörg Müller") //=> "Joerg Mueller"

internet::ip_v4() //=> "222.6.49.200"

internet::private_ip_v4() //=> "172.28.234.148"

internet::ip_v4_cidr() //=> "58.161.221.170/3"

internet::ip_v6() //=> "9c82:cce5:c5a0:c057:bd48:c1ef:d817:379"

internet::ip_v6_cidr() //=> "fe5c:b983:e950:f1ac:7a8d:9eea:7274:f38b/10"

internet::mac_address(None) //=> "00:cc:2d:89:d5:3d"

internet::url(None, None, None) //=> "http://raynor.org/tameka.koelpin"

internet::slug(None, None) //=> "stormy-medhurst"

internet::device_token() //=> "e35f6872910c66aafcd73f1916385b41620fef1fa7c6cd2daff11312aa09dd3b"

internet::user_agent(None) //=> "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; AS; rv:11.0) like Gecko"

internet::bot_user_agent(None) //=> "DuckDuckBot/1.1; (+http://duckduckgo.com/duckduckbot.html)"

internet::base64(16, true, false) //=> "614Y+sBu8tEItGVN"

internet::uuid() //=> "361b2e6c-a2b2-43e3-9883-1d612d5c8103"
```
