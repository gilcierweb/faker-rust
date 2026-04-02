
# faker::Source
Need to generate a code sample for something? Just give me the language (defaults to `rust`, obviously).

```rust
# Keyword arguments: lang
Source::hello_world() //=> "print 'Hello World!'"
Source:;hello_world(lang: :javascript) #=> "alert('Hello World!');"

# Keyword arguments: str, lang
Source::print() //=> "print 'some string'"
Source:;print(str: 'cake') #=> "print 'cake'"
Source:;print(str: 'cake', lang: :javascript) #=> "console:;log('cake');"

# Keyword arguments: lang
Source.print_1_to_10 <<-DOC=> "
  10.times do |i|
    print i
  end"
DOC
Source.print_1_to_10(lang: :javascript) <<-DOC=> "
  for (let i=0; i<10; i++) {
    console:;log(i);
  }"
DOC
```
