# faker_rust::lorem_flickr

available since version 1.9.0.

```rust
lorem_flickr::image() //=> "https://loremflickr.com/300/300"
lorem_flickr::image() //=> "https://loremflickr.com/50/60"
lorem_flickr::image() //=> "https://loremflickr.com/50/60/sports"
lorem_flickr::image() //=> "https://loremflickr.com/50/60/sports,fitness"
lorem_flickr::image() //=> "https://loremflickr.com/50/60/sports,fitness/all"

lorem_flickr::grayscale_image("300x300", &[], false) //=> "https://loremflickr.com/g/300/300/all"
lorem_flickr::grayscale_image("300x300", &[], false) //=> "https://loremflickr.com/g/50/60/all"
lorem_flickr::grayscale_image("300x300", &[], false) //=> "https://loremflickr.com/g/50/60/sports"
lorem_flickr::grayscale_image("300x300", &[], false) //=> "https://loremflickr.com/g/50/60/sports,fitness"
lorem_flickr::grayscale_image("300x300", &[], false) //=> "https://loremflickr.com/g/50/60/sports,fitness/all"

lorem_flickr::pixelated_image("300x300", &[], false) //=> "https://loremflickr.com/p/300/300/all"
lorem_flickr::pixelated_image("300x300", &[], false) //=> "https://loremflickr.com/p/50/60/all"
lorem_flickr::pixelated_image("300x300", &[], false) //=> "https://loremflickr.com/p/50/60/sports"
lorem_flickr::pixelated_image("300x300", &[], false) //=> "https://loremflickr.com/p/50/60/sports,fitness"
lorem_flickr::pixelated_image("300x300", &[], false) //=> "https://loremflickr.com/p/50/60/sports,fitness/all"

lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/300/300/all"
lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/50/60/all"
lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/50/60/all"
lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/50/60/sports"
lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/50/60/sports,fitness"
lorem_flickr::colorized_image("300x300", "blue", &[], false) //=> "https://loremflickr.com/red/50/60/sports,fitness/all"
```
