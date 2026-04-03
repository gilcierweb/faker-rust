# faker_rust::default::driving_licence

```rust
drivinglicence::british_driving_licence        //=> "mcder712081vf7ek"
drivinglicence::british_driving_licence(last_name: "o'carroll",
                                              initials: "j",
                                              gender: :female,
                                              date_of_birth: date::parse("1986-10-24")) //=> "ocarr815246j91ht"

drivinglicence::northern_irish_driving_licence() //=> "70702548"

drivinglicence::uk_driving_licence             //=> "ocarr815246j91ht"
drivinglicence::uk_driving_licence             //=> "70702548"

drivinglicence::usa_driving_licence             //=> "e124590"
drivinglicence::usa_driving_licence('new mexico')       //=> "85793820"
drivinglicence::usa_driving_licence('new mexico')       //=> "57382918"
drivinglicence::usa_driving_licence('new mexico')       //=> "38593028"
