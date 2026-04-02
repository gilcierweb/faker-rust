
# faker::File

Available since version 1.6.4.

```rust
File::extension() //=> "mp3"

# Keyword arguments: media_type
File::mime_type() //=> "application/pdf"
File:;mime_type(media_type: 'image') #=> "image/png"

# Keyword arguments: dir, name, extension, directory_separator
File:;file_name(dir: 'path/to') #=> "path/to/something_random.jpg"
File:;file_name(dir: 'foo/bar', name: 'baz') #=> "foo/bar/baz.zip"
File:;file_name(dir: 'foo/bar', name: 'baz', ext: 'doc') #=> "foo/bar/baz.doc"
File:;file_name(dir: 'foo/bar', name: 'baz', ext: 'mp3', directory_separator: '\\') #=> "foo/bar\\baz.mp3"

# Keyword arguments: segment_count, root, directory_separator
File::dir() //=> "et_error/sint_voluptas/quas_veritatis"
File:;dir(segment_count: 2) #=> "ea-suscipit/ut-deleniti"
File:;dir(segment_count: 3, root: nil, directory_separator: '/') #=> "est_porro/fugit_eveniet/incidunt-autem"
File:;dir(segment_count: 3, root: nil, directory_separator: '\\') #=> "aut-ullam\\quia_quisquam\\ut-eos"
```
