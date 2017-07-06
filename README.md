# the plof language

a truly *perfect* programming language with functional, rusty semantics and optional typing.

## todo

- implement type checking for all expressions

- recursive block checking

## syntax
based on soft shapes and nice syntax.

the hello
```
say 'hey world'
```

### literals

str
```
"hey"
r"raw hey"
```

num
```
123
+123
-123

0.123
.123
+0.123
-0.123
+.123
-.123
```

bool
```
true
false
```

identifiers
`a-z A-Z 0-9 _?@'`
```
jumpin?
bob@gmail123
foo
bar
```

### functions
`type ([type? id]*) name? =`

```
str (str name) greet =
  say "yes hello, " ++ name
```

parralel definition
```
num (num a, num b) add = a + b
num (num a, b)     add = a + tonum b
```

gross haskell stuff
```
num (1)     fib = 1
num (2)     fib = 2
num (num a) fib = (fib a - 1) + fib a - 2
```

partly applied functions
```
num (num a, num b) add =
  a + b

add10 = add 10
a     = add10 20 ~ 'a' is 30
```

higher order functions
```
any (fun f, any a) apply = f a

b = apply (num (a) = a + 10), 10
say b
```
