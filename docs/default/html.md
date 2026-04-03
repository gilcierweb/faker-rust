# faker_rust::default::html

available since version 3.2.1.

the `html` module provides methods for generating random html content.

```rust
html::heading() //=> "<h5>autem</h5>"

html::paragraph() //=> "<p>incidunt atque quis</p>"

html::emphasis() //=> "<em>repellat id impedit.</em>

html::ordered_list() //=> "<ol>\n<li>qui reiciendis non consequatur atque.</li>\n<li>quo doloremque veritatis tempora aut.</li>\n<li>aspernatur.</li>\n<li>ea ab.</li>\n<li>qui.</li>\n<li>sit pariatur nemo eveniet.</li>\n<li>molestiae aut.</li>\n<li>nihil molestias iure placeat.</li>\n<li>dolore autem quisquam.</li>\n</ol>"

html::unordered_list() //=> "<ul>\n<li>voluptatum aliquid tempora molestiae facilis non sed.</li>\n<li>nostrum omnis iste impedit voluptatum dolor.</li>\n<li>esse quidem et facere.</li>\n</ul>"

html::code() //=> "<pre>\n<code>eos quasi qui.</code>\n</pre>"

html::table() //=> "<table>\n<thead>\n<th>ad</th>\n<th>similique</th>\n<th>voluptatem</th>\n</thead>\n<tbody>\n<td>corrupti</td>\n<td>est</td>\n<td>rerum</td>\n<td>molestiae</td>\n<td>quidem</td>\n<td>et</td>\n<td>in</td>\n<td>tempora</td>\n<td>at</td>\n<\tbody>\n<tfoot>\n<td>voluptatem</td>\n<td>debitis</td>\n<td>rem</td>\n</tfoot>\n</table>"

html::script() //=> "<script src=\"http://gulgowski::name/jordan::weimann::js\"></script>"

html::link() //=> "<link rel=\"stylesheet\" href=\"http://fay::io/darryl::barrows::css\">"
html::link() //=> "<link rel=\"icon\" href=\"http://fay::io/darryl::barrows::css\">"

html::element()"}) //=> "<div class=\"xss\" onclick=\"alert('xss')\">this is a div with xss attributes.</div>"

html::random() //=> returns output from a single method outlined above
html::random() //=> returns output from any single method outlined above except for "table"
html::random() //=> returns output from any single method outlined above except for ordered_list and unordered_list

html::sandwich() //=> returns sandwich-style html content as a string
html::sandwich() //=> returns sandwich-style html content with 5 sentences per paragraph and repeated 3 times
```
