# Introduction  
Forge is a programming language primarily based on one sole design philosophy: you have full control over the language, and all aspects of the language are mutable. You control the lexer, parser and even runtime with rules and gradually build your own programming language, specific to the needs of your project.

# Splitmodes  
Splitmodes are one of 2 ways to control the lexer, and the way tokens are generated. By default, Forge's lexer treats each section of code as one token. You control the splitting of tokens with splitmodes. Here is an example:
```forge
~~ e
nnenennn
```  
The first line of code specifies a splitmode, with the character `e`. This will be applied to all of the following code, which will then generate the following tokens: `nn`, `e`, `n`, `e`, and `nnn`.

## Removing rules 
You are able to remove any rule with `/`, including splitmodes.
```forge
~~ x
axa
~~/ x
axa
```  
This will generate the following tokens: `a`, `x`, `a`, and `axa`.
  
## Special splitmodes  
Special splitmodes are splitmodes that don't just match on a single character, but rather have slightly different behavior. The following special splitmodes are available:
- `char`, which splits at every single character
- `whitespace`, which splits at any whitespace

Here are some examples:
```forge
~~ char
fg f
~~/ char
~~ whitespace
ff g fg
```
The first section of code will produce `f`, `g`, ` `and `f` and the second section of code will produce `ff`, `g`, `fg`. Note that for `whitespace`, the whitespace itself will not be included as a token.

# Delimiters  
Delimiters are the second way of controlling the lexer. They work by grouping all text between them as a single token:
```forge
~~ whitespace
^^ ()
hello world
(hello world)
```  
This code would produce the following tokens: `hello`, `world`, `(`, `)`, and `hello world`. 

# Macros
Macros build most functionality and control most things inside your forge program. They are a way to recursively rewrite patterns and create syntax:
```forge
@@ hello => world
hello
```  
This first line of code is a simple example of a macro. It will match on the left side, which is called the pattern, and replace it with the right side. In this case, this macro applies to any occurrence of `hello`. It will then replace this with `world`. So, as you could probably guess, the code on the line next to it will become `world`.

## Capturing tokens  
The pattern of a macro is able to capture a token into a variable:
```
~~ whitespace
@@ hello from $x => $x said hello
hello from forge
```
The final generated code will be `forge said hello`.
Do note that more specific patterns will have a higher priority:
```
~~ whitespace
@@ hello $x => $x hello
@@ hello world => hello forge
hello world
hello forge
```
The rewritten code will be:
```
hello forge
forge hello
```
  
## Generated macros
Macros are able to generate other macros:
```forge
~~ whitespace
@@ hello => @@ world => forge
hello
world
```  
The final generated code will be `forge`.
  
## Macros are recursive  
Macros being recursive means that generated code also gets matched on patterns, until there are no more matches:  
```forge
@@ hello => world
@@ world => forge
hello
```  
This is rewritten to `forge`.
  
## Shadowing macros
A macro can be shadowed by another with the same pattern:
```forge
@@ hello => world
@@ hello => forge
hello
@@/ hello
hello
```  
This code gets rewritten to
```
forge
world
```
Note that removing a macro that shadowed another will make the macro that was shadowed before take effect.

# Runtime primitves
As you might know, it's not possible to create anything from absolutely nothing (except if you're an atheist). This is why Forge has runtime primitives. Runtime primitives are a set of fixed, runtime-native definitions of data. You can view this over at [`docs/runtime-primitves.md`](docs/runtime-primitives.md).
