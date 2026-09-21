This page talks about Bindings, how they work, and how to define them. Their syntax is the following:
```
!! bindings::<binding-name> $a $b $c ... <= <any-token> $a <any-token> $b ...
```
As you can see a binding is essentially a reverse macro. A binding can have a variable number of arguments ($a, $b, ...) depending on the binding's definition.

# Formatting options
Below are some formatting options for bindings and what they mean:
| Syntax | Meaning |
| x|y    | either x or y |
| \e     | nothing (empty string/char) |

# Runtime primitive
Before covering bindings, there is one special binding, known as a runtime primitive. This is the `primitve::num` binding, which has syntax different to normal bindings. It defines what a number is, and takes a prefix and base. The prefix can be defined with the x|y syntax, and the base should be between 1 and 36. Numbers are also the only binding which can not be shadowed but rather can be defined multiple times. Below is an example of possible variants you might implement for numbers:
```
!! primitve::num $pfx $base <= 0x|0X 16
!! primitve::num $pfx $base <= 0b|0B 2
!! primitve::num $pfx $base <= \e 10
```

# Bindings
These are all of the normal bindings and how to define them:

## Bitwise operations
These operate on the runtime primitive `primitive::num`.
| Name | Arguments |
| NOT  | $a        |
| OR   | $a $b     |
| AND  | $a $b     |
| XOR  | $a $b     |
| LSH  | $a $b     |
| RSH  | $a $b     |

## Arithmetic operations
Similar to bitwise operations, these also operate on `primitive::num`.
| Name | Arguments |
| ADD  | $a $b     |
| SUB  | $a $b     |
| MUL  | $a $b     |
| DIV  | $a $b     |
