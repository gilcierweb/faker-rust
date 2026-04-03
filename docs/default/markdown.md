# faker_rust::markdown

available since version 1.8.0.

generates markdown formatting with lorem ipsum text

```rust
markdown::headers() //=> "##### autem"

markdown::emphasis() //=> "_incidunt atque quis repellat id impedit.  quas numquam quod incidunt dicta non. blanditiis delectus laudantium atque reiciendis qui._"

markdown::ordered_list() //=> "1. qui reiciendis non consequatur atque.\n2. quo doloremque veritatis tempora aut.\n3. aspernatur.\n4. ea ab.\n5. qui.\n6. sit pariatur nemo eveniet.\n7. molestiae aut.\n8. nihil molestias iure placeat.\n9. dolore autem quisquam."

markdown::unordered_list() //=> "* voluptatum aliquid tempora molestiae facilis non sed.\n* nostrum omnis iste impedit voluptatum dolor.\n* esse quidem et facere."

markdown::inline_code() //=> "aut eos quis suscipit. `dignissimos voluptatem expedita qui.` quo doloremque veritatis tempora aut."

markdown::block_code() //=> "```rust\neos quasi qui.\n```"

markdown::table() //=> "ad | similique | voluptatem\n---- | ---- | ----\ncorrupti | est | rerum\nmolestiae | quidem | et"

markdown::random() //=> returns output from a single method outlined above

markdown::random("table") //=> returns output from any single method outlined above except for "table"
markdown::random("ordered_list", "unordered_list") //=> returns output from any single method outlined above except for either ordered_list and unordered_list

markdown::sandwich() //=> returns newline separated content of 1 header, 1 default lorem paragraph, and 1 random markdown element
markdown::sandwich() //=> returns newline separated content of 1 header, 1 5-sentence lorem paragraph, and 1 random markdown element
markdown::sandwich() //=> returns newline separated content of 1 header, and then 3 sections consisting of, here, 1 6-sentence lorem paragraph and 1 random markdown element. the random markdown element is chosen at random in each iteration of the paragraph-markdown pairing.
```
