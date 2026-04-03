# faker::books::lovecraft

// Books::Lovecraft

```rust
Books::Lovecraft::fhtagn() //=> "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn"
Books::Lovecraft::fhtagn() //=> "Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn. Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn. Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn"

Books::Lovecraft::deity() //=> "Shub-Niggurath"

Books::Lovecraft::tome() //=> "Book of Eibon"

Books::Lovecraft::location() //=> "Kingsport"

Books::Lovecraft::word() //=> "furtive"

// Keyword arguments: word_count, random_words_to_add, open_compounds_allowed
Books::Lovecraft::sentence() //=> "Furtive antiquarian squamous dank cat loathsome amorphous lurk."
Books::Lovecraft::sentence() //=> "Daemoniac antediluvian fainted squamous comprehension gambrel nameless singular."
Books::Lovecraft::sentence() //=> "Amorphous indescribable tenebrous."
Books::Lovecraft::sentence() //=> "Effulgence unmentionable gambrel."

// Keyword arguments: number, spaces_allowed
Books::Lovecraft::words() //=> ["manuscript", "abnormal", "singular"]
Books::Lovecraft::words() //=> ["daemoniac", "cat"]
Books::Lovecraft::words() //=> ["lurk", "charnel"]

// Keyword arguments: number
Books::Lovecraft::sentences() //=> ["Nameless loathsome decadent gambrel.", "Ululate swarthy immemorial cat madness gibbous unmentionable unnamable.", "Decadent antediluvian non-euclidean tentacles amorphous tenebrous."]
Books::Lovecraft::sentences() //=> ["Antediluvian amorphous unmentionable singular accursed squamous immemorial.", "Gambrel daemoniac gibbous stygian shunned ululate iridescence abnormal."]

// Keyword arguments: sentence_count, random_sentences_to_add
Books::Lovecraft::paragraph() //=> "Squamous nameless daemoniac fungus ululate. Cyclopean stygian decadent loathsome manuscript tenebrous. Foetid abnormal stench. Dank non-euclidean comprehension eldritch. Charnel singular shunned lurk effulgence fungus."
Books::Lovecraft::paragraph() //=> "Decadent lurk tenebrous loathsome furtive spectral amorphous gibbous. Gambrel eldritch daemoniac cat madness comprehension stygian effulgence."
Books::Lovecraft::paragraph() //=> "Stench cyclopean fainted antiquarian nameless. Antiquarian ululate tenebrous non-euclidean effulgence."

// Keyword arguments: number
Books::Lovecraft::paragraphs() //=> ["Noisome daemoniac gibbous abnormal antediluvian. Unutterable fungus accursed stench noisome lurk madness indescribable. Antiquarian fungus gibbering lurk dank fainted. Hideous loathsome manuscript daemoniac lurk charnel foetid.", "Non-euclidean immemorial indescribable accursed furtive. Dank unnamable cyclopean tenebrous stench immemorial. Eldritch abnormal gibbering tenebrous. Singular accursed lurk.", "Charnel antediluvian unnamable cat blasphemous comprehension tenebrous. Nameless accursed amorphous unnamable stench. Squamous unnamable mortal accursed manuscript spectral gambrel amorphous. Shunned stygian charnel unutterable. Tenebrous ululate lurk amorphous unnamable."]
Books::Lovecraft::paragraphs() //=> ["Hideous amorphous manuscript antediluvian non-euclidean cat eldritch foetid. Stench squamous manuscript amorphous gibbering fainted gibbous. Accursed loathsome blasphemous iridescence antediluvian abnormal ululate manuscript. Singular manuscript gibbering decadent accursed indescribable.", "Tenebrous unnamable comprehension antediluvian lurk. Lurk spectral noisome gibbering. Furtive manuscript madness tenebrous daemoniac."]

// Keyword arguments: characters, supplemental
Books::Lovecraft::paragraph_by_chars() //=> "Truffaut stumptown trust fund 8-bit messenger bag portland. Meh kombucha selvage swag biodiesel. Lomo kinfolk jean shorts asymmetrical diy. Wayfarers portland twee stumptown. Wes anderson biodiesel retro 90's pabst. Diy echo 90's mixtape semiotics. Cornho."
Books::Lovecraft::paragraph_by_chars() //=> "Hella kogi blog narwhal sartorial selfies mustache schlitz. Bespoke normcore kitsch cred hella fixie. Park aesthetic fixie migas twee. Cliche mustache brunch tumblr fixie godard. Drinking pop-up synth hoodie dreamcatcher typewriter. Kitsch biodiesel green."
```
