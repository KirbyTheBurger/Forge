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

# Grouping delimiters
Grouping delimiters are a way of controlling the order of macro expansion. The innermost pair of grouping delimiters has the highest priority and gets expanded first:
```
%% ()
~~ whitespace
@@ foo bar => forge
@@ bar foo => world
foo (bar foo)
```
This gets rewritten to `foo world`.

# Bindings
This section talks about Bindings, how they work, and how to define them. Their syntax is the following:
```
!! bindings::<binding-name> $a $b $c ... <= <any-token> $a <any-token> $b ...
```
As you can see a binding is essentially a reverse macro. A binding can have a variable number of arguments ($a, $b, ...) depending on the binding's definition.

## Formatting options
Below are some formatting options for bindings and what they mean:
| Syntax | Meaning |
| x|y    | either x or y |
| \e     | nothing (empty string/char) |

## Runtime primitive
Before covering bindings, there is one special binding, known as a runtime primitive. This is the `primitve::num` binding, which has syntax different to normal bindings. It defines what a number is, and takes a prefix and base. The prefix can be defined with the x|y syntax, and the base should be between 1 and 36. Numbers are also the only binding which can not be shadowed but rather can be defined multiple times. Below is an example of possible variants you might implement for numbers:
```
!! primitve::num $pfx $base <= 0x|0X 16
!! primitve::num $pfx $base <= 0b|0B 2
!! primitve::num $pfx $base <= \e 10
```

## List of bindings
These are all of the normal bindings and how to define them:

### Bitwise operations
These operate on the runtime primitive `primitive::num`.
| Name | Arguments |
|------|-----------|
| NOT  | $a        |
| OR   | $a $b     |
| AND  | $a $b     |
| XOR  | $a $b     |
| LSH  | $a $b     |
| RSH  | $a $b     |

### Arithmetic operations
Similar to bitwise operations, these also operate on `primitive::num`.
| Name | Arguments |
|------|-----------|
| ADD  | $a $b     |
| SUB  | $a $b     |
| MUL  | $a $b     |
| DIV  | $a $b     |
