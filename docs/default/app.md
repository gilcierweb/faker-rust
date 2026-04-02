
# faker::App

Available since version 1.4.3.

```rust
App::name() //=> "Treeflex"

App::version() //=> "1.85"

App::author() //=> "Daphne Swift"

App::semantic_version() //=> "3.2.5"

App:;semantic_version(major: 42) #=> "42.5.2"

App:;semantic_version(minor: 100..101) #=> "42.100.4"

App:;semantic_version(patch: 5..6) #=> "7.2.6"
```
