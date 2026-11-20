# faker_rust::string

random utf-8 string with optional nested length selectors.  very useful for testing.  guaranteed to find bugs in your code.  want robust code?  test with these.  includes every utf-8 codepoint.

```rust
string::random(8) //=> "3 뇦\u0017&y\u{3a109}$8^4* 녹豿4좘툢ꔾ쉙6ɉ\ua6 8tn畀챵|\"3쇤ŵ"
string::random(8) //=> "⼨%0*"
string::random(8) //=> "\u{69fdc};秨툫"
string::random(8) //=> "i轤𣴒p溟l"
string::random(8) //=> "葓l#ћ"
```
