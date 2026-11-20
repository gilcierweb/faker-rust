# faker_rust::source

need to generate a code sample for something? just give me the language (defaults to `rust`, obviously).

```rust
source::hello_world("rust") //=> "print 'hello world!'"
source::hello_world("rust") //=> "alert('hello world!');"

source::print("hello", "rust") //=> "print 'some string'"
source::print("hello", "rust") //=> "print 'cake'"
source::print("hello", "rust") //=> "console::log('cake');"

source::print_1_to_10 <<-doc=> "
  10.times do |i|
    print i
  end"
doc
source::print_1_to_10("rust") <<-doc=> "
  for (let i=0; i<10; i++) {
    console::log(i);
  }"
doc
```
