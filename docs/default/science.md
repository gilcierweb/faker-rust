
# faker::Science

```rust
##
# Produces a name of a science
# You can optionally filter by specifying one or more of the following:
# `:empirical, :formal, :natural, :social, :basic, :applied`
# @see https://en.wikipedia.org/wiki/Science#Branches_of_science
# @see Educator.subject
Science::science() //=> "Space science"
Science:;science(:natural, :applied) #=> "Engineering"
Science:;science(:formal, :applied) #=> "Computer Science"

Science::element() //=> "Carbon"

Science::element_symbol() //=> "Pb"

Science::element_state() //=> "Liquid"

Science::element_subcategory() //=> "Reactive nonmetal"

Science::scientist() //=> "Isaac Newton"

Science::scientist() //=> "Isaac Newton"

Science::modifier() //=> "Quantum"

##
# Produces the name of a scientific tool.
# Optionally it can generate tools with a science word modifier that sound more fancy.
#
# @param simple [Boolean] Whether to generate fancy non-realistic tool names, using the Q-word for example.
# @return [String]
Science::tool() //=> "Superconductive Microcentrifuge"
Science:;tool(simple: true) #=> "Microcentrifuge"
```
