
# faker::String
Random UTF-8 string with optional nested length selectors.  Very useful for testing.  Guaranteed to find bugs in your code.  Want robust code?  Test with these.  Includes every UTF-8 codepoint.

```rust
# Keyword arguments: length
String::random() //=> "3 뇦\u0017&y\u{3A109}$8^4* 녹豿4좘툢ꔾ쉙6ɉ\uA6 8TN畀챵|\"3쇤Ŵ"
String:;random(length: 4) #=> "⼨%0*"
String:;random(length: 3..12) #=> "\u{69FDC};秨툫"
String:;random(length: [0, 6]) #=> "I轤𣴒P溟L"
String:;random(length: [1, (2..5), [3, 6], nil]) #=> "葓L#ћ"
```
