# Introduction  
Forge is a language which starts with the absolute bare minimum: numbers, bitwise operations, rules and macros.
Through the use of macros, the user gradually builds a language completely specific to their project.
You can edit the lexer to behave differently based on rules you set, and macros you define.

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

## Capturing variables  
When matching patterns, you can capture a token as a variable:  
```forge
@@ hello $x => $x world
hello forge
```  
`hello $x` just means "look for any syntax that follows the pattern hello and then any token, and assign that value to x". So, the generated code would be `forge world`.  
  
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
This code makes the lexer create a new token each time it sees the letter e (and won't include the letter itself).
For example, if you write the code `nnenennn`, the lexer would produce 3 tokens: `nn`, `n` and `nnn`.  