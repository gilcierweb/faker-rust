# faker::default::source

need to generate a code sample for something? just give me the language (defaults to `rust`, obviously).

```rust
source::hello_world() //=> "print 'hello world!'"
source::hello_world() //=> "alert('hello world!');"

source::print() //=> "print 'some string'"
source::print() //=> "print 'cake'"
source::print() //=> "console::log('cake');"

source::print_1_to_10 <<-doc=> "
  10.times do |i|
    print i
  end"
doc
source::print_1_to_10() <<-doc=> "
  for (let i=0; i<10; i++) {
    console::log(i);
  }"
doc
```
