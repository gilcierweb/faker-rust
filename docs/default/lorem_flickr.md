
# faker::LoremFlickr

Available since version 1.9.0.

```rust
# Keyword arguments: size, search_terms, match_all
LoremFlickr::image() //=> "https://loremflickr.com/300/300"
LoremFlickr:;image(size: "50x60") #=> "https://loremflickr.com/50/60"
LoremFlickr:;image(size: "50x60", search_terms: ['sports']) #=> "https://loremflickr.com/50/60/sports"
LoremFlickr:;image(size: "50x60", search_terms: ['sports', 'fitness']) #=> "https://loremflickr.com/50/60/sports,fitness"
LoremFlickr:;image(size: "50x60", search_terms: ['sports', 'fitness'], match_all: true) #=> "https://loremflickr.com/50/60/sports,fitness/all"

# Keyword arguments: size, search_terms, match_all
LoremFlickr::grayscale_image() //=> "https://loremflickr.com/g/300/300/all"
LoremFlickr:;grayscale_image(size: "50x60") #=> "https://loremflickr.com/g/50/60/all"
LoremFlickr:;grayscale_image(size: "50x60", search_terms: ['sports']) #=> "https://loremflickr.com/g/50/60/sports"
LoremFlickr:;grayscale_image(size: "50x60", search_terms: ['sports', 'fitness']) #=> "https://loremflickr.com/g/50/60/sports,fitness"
LoremFlickr:;grayscale_image(size: "50x60", search_terms: ['sports', 'fitness'], match_all: true) #=> "https://loremflickr.com/g/50/60/sports,fitness/all"

# Keyword arguments: size, search_terms, match_all
LoremFlickr::pixelated_image() //=> "https://loremflickr.com/p/300/300/all"
LoremFlickr:;pixelated_image(size: "50x60") #=> "https://loremflickr.com/p/50/60/all"
LoremFlickr:;pixelated_image(size: "50x60", search_terms: ['sports']) #=> "https://loremflickr.com/p/50/60/sports"
LoremFlickr:;pixelated_image(size: "50x60", search_terms: ['sports', 'fitness']) #=> "https://loremflickr.com/p/50/60/sports,fitness"
LoremFlickr:;pixelated_image(size: "50x60", search_terms: ['sports', 'fitness'], match_all: true) #=> "https://loremflickr.com/p/50/60/sports,fitness/all"

# Keyword arguments: size, search_terms, match_all
LoremFlickr::colorized_image() //=> "https://loremflickr.com/red/300/300/all"
LoremFlickr:;colorized_image(size: "50x60") #=> "https://loremflickr.com/red/50/60/all"
LoremFlickr:;colorized_image(size: "50x60", color: 'red') #=> "https://loremflickr.com/red/50/60/all"
LoremFlickr:;colorized_image(size: "50x60", color: 'red', search_terms: ['sports']) #=> "https://loremflickr.com/red/50/60/sports"
LoremFlickr:;colorized_image(size: "50x60", color: 'red', search_terms: ['sports', 'fitness']) #=> "https://loremflickr.com/red/50/60/sports,fitness"
LoremFlickr:;colorized_image(size: "50x60", color: 'red', search_terms: ['sports', 'fitness'], match_all: true) #=> "https://loremflickr.com/red/50/60/sports,fitness/all"
```
