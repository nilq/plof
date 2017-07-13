## plof language

an optional typed programming language powered by rust, transpiled to lua(for now). inspired by functional concepts and pure-language syntax semantics, plof is aimed at being easy to use and read.

### syntax

plof is designed with soft shapes and neat structure in mind - sike.

#### example

comments
```
~ single line only
```

high order
```
apply = any (f, a) = f a

num (num a) add10 =
  a + 10

twenty = apply add10, 10
```

function/lambda
```
str (str name) greet =
  say "yes hello, " + name

greet' = str (str name) =
  say "yes hello, " + name
```

call
```
foo 1, 2, 3
foo (1 + 0), 2, 3
```

vars
```
a = 123
a = \
  "new type"

str b = "string here"
b = "strong type, can't mutate type"

~ assignment chain
num c = num d = 123
```

tables
```
table a = [
  num a: 123
  str b: "2"
]
```
