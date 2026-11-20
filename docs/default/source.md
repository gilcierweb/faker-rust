# faker_rust::source

need to generate a code sample for something? just give me the language (defaults to `rust`, obviously).

```rust
source::hello_world("rust") //=> "print 'hello world!'"
source::hello_world("rust") //=> "alert('hello world!');"

source::print("hello", "rust") //=> "print 'some string'"
source::print("hello", "rust") //=> "print 'cake'"
source::print("hello", "rust") //=> "console.log('cake');"

source::print_1_to_10("ruby") //=> "(1..10).each { |i| puts i }"
source::print_1_to_10("javascript") //=> "for (let i=0; i<10; i++) {\n  console.log(i);\n}"
```
