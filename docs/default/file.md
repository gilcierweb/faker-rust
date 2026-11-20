# faker_rust::file

available since version 1.6.4.

```rust
file::extension() //=> "mp3"

file::mime_type() //=> "application/pdf"
file::mime_type() //=> "image/png"

file::file_name(None, None) //=> "path/to/something_random::jpg"
file::file_name(None, None) //=> "foo/bar/baz::zip"
file::file_name(None, None) //=> "foo/bar/baz::doc"
file::file_name(None, None) //=> "foo/bar\\baz::mp3"

file::dir(3, None, "/") //=> "et_error/sint_voluptas/quas_veritatis"
file::dir(3, None, "/") //=> "ea-suscipit/ut-deleniti"
file::dir(3, None, "/") //=> "est_porro/fugit_eveniet/incidunt-autem"
file::dir(3, None, "/") //=> "aut-ullam\\quia_quisquam\\ut-eos"
```
