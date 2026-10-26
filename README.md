# Roadmap
Attempt to write a untyped lambda calculus compiler in Rust.  
the project currently has a lexer, a parser, a semantic analyzer, and an evaluator.  

the evaluator is to be later replaced with a IR generator for the LLVM IR.  
standard library bindings and multi file support remains to be added

# To get started
create a text file (extension does not matter yet)

## the syntax is as follows
> Create abstractions with either `lambda` followed by a space and then the identifier or with the unicode character U+033B `λ` followed by the identifier.  

```
lambda x.x;
λx.x;
```

end each line with a semicolon `;`

> Applications can be written as expressions next to each other.

```
λx.x y;
```
> expressions can be enclosed in brackets.
```
(λb.λx.λx) b x y;
(λb.λx.λy) b x y;
```

> This implementation allows the creation of named expressions or abstractions using the `=` operator, with a name on the left and the expression on the right.
```
TRUE = λx.λy.x;
FALSE = λx.λy.y;
IF = λb.λx.λy.b x y;
```

## Compiler settings and usage
To use the compiler run the compiler passing the location of the source file from the current directory.  

for example if the file is in a directory called test in the current directory pass the arguments.  
``` -s test\source.lc```

other compiler flags are
- -d (gives debug info by printing each reduction step of each statement)
- -f (tells the semantic analyzer to allow free variables) if you get an `undeclared identifier: ` error try running the compiler with this flag
- -pd (skip printing definition nodes)

### example compiler usage

windows
```.\lambda_compiler.exe -s tests\source.lc -f -pd```

## Pipeline

currently the pretty printer just goes over the evaluated AST and forms a string visiting each node,
this string is printed ones the pretty printer goes over the entire AST.  
currently this is the only way to interface with the compiler since File Handling, IO and Networking abstractions have not been made yet,
they are to be added when the standard library is created

## Details
the compiler uses the lazy evaluation model for normalization, shadowing variables is permitted as they will be alpha substituted during beta reduction lazily.  
the compiler makes sure to only alpha rename and beta substitution would cause a free variable to become bound. 

## Current State
the language is Turing complete and implements the standard untyped lambda calculus properly, so recursion using combinators such as Y-Combinator , if statements, church numerals,
 integers using church numeral pairs, are all possible.

 please give it a try.


