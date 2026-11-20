# faker_rust::lorem

```rust
lorem::word() //=> "repellendus"
lorem::word() //=> "nisi"
lorem::word() //=> "et"
lorem::word() //=> "consequatur"

lorem::words(3) //=> ["dolores", "adipisci", "nesciunt"]
lorem::words(3) //=> ["culpa", "recusandae", "aut", "omnis"]
lorem::words(3) //=> ["colloco", "qui", "vergo", "deporto"]

lorem::multibyte() //=> 😀

lorem::words(3) //=> ["error", "cum", "nesciunt"]
lorem::words(3) //=> ["nisi", "allatus", "consequatur", "aut"]
lorem::words(3) //=> ["nisi", "allatus", "consequatur", "aut"]
lorem::words(3) //=> ["nisi", "allatus", "consequatur", "aut"]

lorem::characters() //=> "uw1ep04lhs0c4d931n1jmrspprf5wrj85fefue0y7y6m56b6omquh7br7dhqijwlawejpl765nb1716idmp3xnfo85v349pzy2o9rir23y2qhflwr71c1585fnynguiphkjm8p0vktwitcsm16lny7jzp9t4drwav3qmhz4yjq4k04x14gl6p148hulyqioo72tf8nwrxxcclfypz2lc58lsibgfe5w5p0xv95peafjjmm2frkhdc6duoky0aha"
lorem::characters() //=> "ang9cbhoa8"
lorem::characters() //=> "ang9cbhoa8"
lorem::characters() //=> "ang9cbhoa8"

lorem::sentence() //=> "quia illum libero magni."
lorem::sentence() //=> "quaerat quam unde."
lorem::sentence() //=> "vehemens velit cogo."
lorem::sentence() //=> "aut voluptatem illum fugit ut sit."
lorem::sentence() //=> "accusantium tantillus dolorem timor."

lorem::sentences(None, None) //=> ["vero earum commodi soluta.", "quaerat fuga cumque et vero eveniet omnis ut.", "cumque sit dolor ut est consequuntur."]
lorem::sentences(None, None) //=> ["ut perspiciatis explicabo possimus doloribus enim quia."]
lorem::sentences(None, None) //=> ["quis capillus curo ager veritatis voro et ipsum."]

lorem::paragraph() //=> "recusandae minima consequatur. expedita sequi blanditiis. ut fuga et."
lorem::paragraph() //=> "ducimus mollitia deserunt. dicta et corporis."
lorem::paragraph() //=> "strues amplitudo dignissimos. ver trucido charisma."
lorem::paragraph() //=> "neque aut et nemo aut incidunt voluptates. dolore cum est sint est. vitae assumenda porro odio dolores fugiat. est voluptatum quia rerum."
lorem::paragraph() //=> "vomito unde uxor annus. et patior utilis sursum."

lorem::paragraphs(None, None) //=> ["dolores quis quia ad quo voluptates. maxime delectus totam numquam. necessitatibus vel atque qui dolore.", "id neque nemo. dolores iusto facere est ad. accusamus ipsa dolor ut.", "et officiis ut hic. sunt asperiores minus distinctio debitis ipsa dolor. minima eos deleniti."]
lorem::paragraphs(None, None) //=> ["labore voluptas sequi. ratione nulla eaque quia molestiae fugit. at quam laboriosam aut ut dignissimos."]
lorem::paragraphs(None, None) //=> ["depulso animi cunctatio amicitia adficio. vester viduo qui despirmatio voluptas. validus laudantium adopto ut agnitio venustas. aer arcus odio esse."]

lorem::question(5, true, 2, None) //=> "vero eum dolore qui?"
lorem::question(5, true, 2, None) //=> "eos temporibus necessitatibus labore?"
lorem::question(5, true, 2, None) //=> "eum ut repellendus neque?"
lorem::question(5, true, 2, None) //=> "expedita quia labore voluptatem?"

lorem::questions(3, true) //=> ["ratione eaque doloremque?", "qui aliquid fuga?", "est cum ipsum?"]
lorem::questions(3, true) //=> ["iusto ipsa magnam?", "quibusdam eaque amet?", "officia delectus laboriosam?"]

lorem::paragraph_by_chars(200, true) //=> "truffaut stumptown trust fund 8-bit messenger bag portland. meh kombucha selvage swag biodiesel. lomo kinfolk jean shorts asymmetrical diy. wayfarers portland twee stumptown. wes anderson biodiesel retro 90's pabst. diy echo 90's mixtape semiotics. cornho."
lorem::paragraph_by_chars(200, true) //=> "hella kogi blog narwhal sartorial selfies mustache schlitz. bespoke normcore kitsch cred hella fixie. park aesthetic fixie migas twee. cliche mustache brunch tumblr fixie godard. drinking pop-up synth hoodie dreamcatcher typewriter. kitsch biodiesel green."
```
