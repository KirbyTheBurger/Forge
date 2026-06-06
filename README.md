# Introduction  
Forge is a language which starts with the absolute bare minimum: rules, macros, and some built in logic.
Through the use of macros, the user gradually builds a language completely specific to their project.
You can edit the lexer to behave differently based on rules you set, and macros you define. Do note that all of the things discussed in this documentation are subject to change as Forge is unfinished and some features discussed haven't even been partially implemented.

# Macros
Macros are the core functionality of Forge. They are used to rewrite code and define syntax.  
```forge
@@ hello => world
```  
Let's go over this snippet of code. This is a macro that replaces anything that matches the pattern `hello` (i.e. every ocurrance of `hello`) with `world`. The `@@` is used to mark a line as a macro. If, and only if the first 2 characters of any line are `@@`, that means that it is used to declare a macro. The `=>` is just a symbol that means *replace*, so in this case `hello` gets replaced by `world`.  
So, if we were to add some code on the next line:
```forge
@@ hello => world
hello
```  
This would mean that the code generated at compile time will be `world`, since hello gets replaced with world by the macro we defined.  

## Capturing values  
When matching patterns, you can capture a token as a variable:  
```forge
@@ hello $x => $x world
hello forge
```  
`hello $x` just means "look for any syntax that follows the pattern hello and then any token, and assign that value to x". So, the generated code would be `forge world`. The name of a capture value can only be 1 character.  
  
## Generating other macros  
It is possible to generate other macros from macros:  
```forge
@@ hello => @@ world => forge
hello
world
```  
This replaces any ocurrance of `hello` with the macro `@@ world => forge`, which replaces any ocurrance of `world` with `forge`. The generated code would be `forge`.  
  
## Macros are recursive  
Macros being recursive means that generated code also gets matched on patterns, until there are no more matches:  
```forge
@@ hello => world
@@ world => forge
hello
```  
These macros first replace `hello` with `world`, and then, because a pattern matches with the generated code, `world` gets replaced by `forge`. The final generated code is `forge`.  
  
## Removing macros  
You're able to remove macros using `@@/`. This means that after the line where you remove the macro, it won't get enforced as a pattern anymore:  
```forge
@@ hello => world
hello
@@/ hello
hello
```  
The final generated code is:  
```forge
world
hello
```  
This is because on the second line, the macro that replaces `hello` with `world` is active, but on the third line it gets removed, resulting in the second `hello` not being replaced by anything.  
  
## Shadowing macros  
Macros can be shadowed by macros with the same pattern. This will remove the earlier macro and redefine it as something else:  
```forge
@@ hello => world
@@ hello => forge
hello
```  
The resulting generated code would be `forge`, since the first macro gets shadowed by the second one.  
  
# Splitmodes  
By default, the lexer treats each section of code as a single token. To change this functionality,
you can define a splitmode to split on a specific character or string of characters:  
```forge
~~ e
```  
This code makes the lexer create a new token each time it sees the letter e (and create a separate token of the split character).
For example, if you write the code `nnenennn`, the lexer would produce 5 tokens: `nn`, `e`, `n`, `e` and `nnn`.  
  
## Special splitmodes  
Special splitmodes are splitmodes that don't just match on a single character, but rather have slightly different behavior:  
```forge
~~ char
```  
This makes the lexer treat each individual charachter as 1 token. For example, this would split the code `forge` into the tokens `f`, `o`, `r`, `g` and `e`.
```forge
~~ whitespace
```  
This splitmode just splits at every whitespace. `hello world` would get split into the tokens `hello` and `world`. Whitespace is the only splitmode that doesnt include the split character as a token. In a lot of the code examples in this file, it is assumed that the whitespace splitmode is enabled for showcasing purposes.  
  
## Removing splitmodes 
Just like with macros, you can remove a splitmode by putting a slash behind the `~~`:  
```forge
~~ whitespace
hello world
~~/ whitespace
hello world
```  
This would generate the tokens `hello`, `world` and `hello world`.  
  
# Delimiters  
Delimiters are somewhat similar to splitmodes, however they come in a pair and anything between them gets parsed as 1 whole token. You create a new delimiter pair using `^^`.  
```forge
~~ whitespace
^^ ()
hello world
(hello world)
```  
This would produce the tokens `hello`, `world`, `(`, `hello world` and `)`. Since the second `hello world` is in between a delimiter pair, it gets parsed as 1 whole token, ignoring the whitespace splitmode.  
  
## Removing delimiters  
Obviously you can also remove delimiters using `^^/`:  
```forge
^^ ()
(1)
^^/ ()
(1)
```  
This produces the tokens `(`, `1`, `)` and `(1)`.  
  
# Built in logic  
As you might know, you can't create anything with absolutely nothing (except if you believe in evolution). Because of this, Forge comes with a few minimal built in things that allow anything to be created.  
  
## Numbers and bitwise operations  
To make manipulating data possible, numbers and bitwise operations are one of the minimal functionalities of Forge. The following bitwise operations are available:  
- `_AND_`
- `_OR_`
- `_NOT_`
- `_XOR_`
- `_SHL_` (shift left)
- `_SHR_` (shift right)
  
You can only use these operations on tokens that fully consist of numbers, and those are treated as decimal numbers (bitwise operations get applied to the binary equivalent). Bitwise operations are done in the form 
`[operation] $x $y`. For example:  
```forge
_AND_ 0 1
```  
This would compute to 0 (since 0 & 1 = 0). You can also, for example, rewrite this using a macro:  
```forge
@@ $x & $y => _AND_ $x $y
```  
  
## I/O  
Work in progress  
  
## Examples  
The following are some simple examples of what you can do with Forge. See more inside of the `examples` folder (w.i.p.).  
