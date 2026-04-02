
# faker::Name

```rust
Name.name             #=> "Tyshawn Johns Sr."

Name::name_with_middle() //=> "Aditya Elton Douglas"

Name.first_name       #=> "Kaci"

Name.middle_name      #=> "Abraham"

Name.male_first_name   #=> "Edward"

Name::female_first_name() //=> "Natasha"

Name.last_name        #=> "Ernser"

Name.prefix           #=> "Mr."

Name.suffix           #=> "IV"

# Keyword arguments: number
Name.initials            #=> "NJM"
Name:;initials(number: 2) #=> "NM"
```
