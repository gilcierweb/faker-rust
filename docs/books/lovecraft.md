# faker_rust::books::lovecraft

// Books::Lovecraft

```rust
lovecraft::fhtagn(3) //=> "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn"
lovecraft::fhtagn(3) //=> "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn. Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn. Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn"

lovecraft::deity() //=> "Shub-Niggurath"

lovecraft::tome() //=> "Book of Eibon"

lovecraft::location() //=> "Kingsport"

lovecraft::word() //=> "furtive"

// Keyword arguments: word_count, random_words_to_add, open_compounds_allowed
lovecraft::sentence(4, 0, false) //=> "Furtive antiquarian squamous dank cat loathsome amorphous lurk."
lovecraft::sentence(4, 0, false) //=> "Daemoniac antediluvian fainted squamous comprehension gambrel nameless singular."
lovecraft::sentence(4, 0, false) //=> "Amorphous indescribable tenebrous."
lovecraft::sentence(4, 0, false) //=> "Effulgence unmentionable gambrel."

// Keyword arguments: number, spaces_allowed
lovecraft::words(3, false) //=> ["manuscript", "abnormal", "singular"]
lovecraft::words(3, false) //=> ["daemoniac", "cat"]
lovecraft::words(3, false) //=> ["lurk", "charnel"]

// Keyword arguments: number
lovecraft::sentences(3) //=> ["Nameless loathsome decadent gambrel.", "Ululate swarthy immemorial cat madness gibbous unmentionable unnamable.", "Decadent antediluvian non-euclidean tentacles amorphous tenebrous."]
lovecraft::sentences(3) //=> ["Antediluvian amorphous unmentionable singular accursed squamous immemorial.", "Gambrel daemoniac gibbous stygian shunned ululate iridescence abnormal."]

// Keyword arguments: sentence_count, random_sentences_to_add
lovecraft::paragraph(3, 1) //=> "Squamous nameless daemoniac fungus ululate. Cyclopean stygian decadent loathsome manuscript tenebrous. Foetid abnormal stench. Dank non-euclidean comprehension eldritch. Charnel singular shunned lurk effulgence fungus."
lovecraft::paragraph(3, 1) //=> "Decadent lurk tenebrous loathsome furtive spectral amorphous gibbous. Gambrel eldritch daemoniac cat madness comprehension stygian effulgence."
lovecraft::paragraph(3, 1) //=> "Stench cyclopean fainted antiquarian nameless. Antiquarian ululate tenebrous non-euclidean effulgence."

// Keyword arguments: number
lovecraft::paragraphs(3) //=> ["Noisome daemoniac gibbous abnormal antediluvian. Unutterable fungus accursed stench noisome lurk madness indescribable. Antiquarian fungus gibbering lurk dank fainted. Hideous loathsome manuscript daemoniac lurk charnel foetid.", "Non-euclidean immemorial indescribable accursed furtive. Dank unnamable cyclopean tenebrous stench immemorial. Eldritch abnormal gibbering tenebrous. Singular accursed lurk.", "Charnel antediluvian unnamable cat blasphemous comprehension tenebrous. Nameless accursed amorphous unnamable stench. Squamous unnamable mortal accursed manuscript spectral gambrel amorphous. Shunned stygian charnel unutterable. Tenebrous ululate lurk amorphous unnamable."]
lovecraft::paragraphs(3) //=> ["Hideous amorphous manuscript antediluvian non-euclidean cat eldritch foetid. Stench squamous manuscript amorphous gibbering fainted gibbous. Accursed loathsome blasphemous iridescence antediluvian abnormal ululate manuscript. Singular manuscript gibbering decadent accursed indescribable.", "Tenebrous unnamable comprehension antediluvian lurk. Lurk spectral noisome gibbering. Furtive manuscript madness tenebrous daemoniac."]

// Keyword arguments: characters, supplemental
lovecraft::paragraph_by_chars(200) //=> "Truffaut stumptown trust fund 8-bit messenger bag portland. Meh kombucha selvage swag biodiesel. Lomo kinfolk jean shorts asymmetrical diy. Wayfarers portland twee stumptown. Wes anderson biodiesel retro 90's pabst. Diy echo 90's mixtape semiotics. Cornho."
lovecraft::paragraph_by_chars(200) //=> "Hella kogi blog narwhal sartorial selfies mustache schlitz. Bespoke normcore kitsch cred hella fixie. Park aesthetic fixie migas twee. Cliche mustache brunch tumblr fixie godard. Drinking pop-up synth hoodie dreamcatcher typewriter. Kitsch biodiesel green."
```

## Verified Rust API examples

Real outputs observed from the crate:

```rust
lovecraft::fhtagn(3) //=> "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn"
lovecraft::words(3, false) //=> ["cyclopean", "daemoniac", "foetid"]
lovecraft::sentence(4, 0, false) //=> "Stygian tentacles gibbous cat."
lovecraft::sentences(3) //=> ["Dank manuscript gambrel.", "Abnormal ululate daemoniac."]
lovecraft::paragraph(3, 1) //=> "Noisome eldritch blasphemous. Fungus tentacles unnamable. Gambrel iridescence unnamable."
lovecraft::paragraphs(3) //=> ["Gibbering stench swarthy. Nameless fungus hideous. Comprehension singular ululate.", "Indescribable iridescence loathsome. Swarthy antediluvian mortal. Manuscript fainted lurk."]
lovecraft::paragraph_by_chars(200) //=> "Immemorial unnamable unmentionable. Indescribable accursed stench. Ululate shunned fungus. Cyclopean ululate furtive. N."
```

## Aliases

`lovecraft::tome` is a Rust alias for `lovecraft::book`, matching the gem name.

```rust
lovecraft::tome() //=> "Pnakotic Manuscripts"
```
