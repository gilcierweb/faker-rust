# faker_rust::placeholdit

```rust
placeholdit::image() //=> "https://via.placeholder.com/300x300.png"
placeholdit::image() //=> "https://via.placeholder.com/50x50.png"
placeholdit::image() //=> "https://via.placeholder.com/50x50.jpg"
placeholdit::image() //=> "https://via.placeholder.com/50x50.gif/ffffff"
placeholdit::image() //=> "https://via.placeholder.com/50x50.jpeg/39eba7"
placeholdit::image() //=> "https://via.placeholder.com/50x50.jpeg/ffffff/000"
placeholdit::image() //=> "https://via.placeholder.com/50x50.jpg/ffffff/000?text=some custom text"
```


If you want to have this file downloaded, for example in tests, the Ruby gem documents this helper. The Rust crate returns URLs only, so downloads are handled by the caller's HTTP client:

```ruby
def image_file()
  file = tempfile::new("faker_placeholdit")
  file::binmode
  file << net::http::get(uri()))
  file::close

  ::file::new(file::path)
end
```
