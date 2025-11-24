# Language Features

## Currently supported literals

### Strings
String literals can be created using the typical `"text"` syntax.
```"Hello";```

Strings also support all common escape sequences (and a few less common ones, like `"\a"`). All
supported escape sequences are:
- `"\n"` -> Newline
- `"\t"` -> Tab / Horizontal tab
- `"\r"` -> Carriage Return
- `"\b"` -> Backspace
- `"\0"` -> Null Byte
- `"\f"` -> Form Feed
- `"\v"` -> Vertical Tab
- `"\a"` -> Terminal Bell / Alert
- `"\u{...}"` -> Unicode Code Points (only hexadecimal characters allowed inside {...})
- `"\x##"` -> ASCII Code Points (only hexadecimal characters allowed as #, up to `7F`)

Putting a `\` before any other character just adds that character to the string, meaning to do
a literal backslash you do `\\`, for a literal double quote you do `\"`, and something like `\c`
just ends up being the same as if you put a literal `c` there.

Strings can also be concatenated with other strings using the `+` (Add) operator.
```
"Hello" + " " + "World";
>> "Hello World"
```

Strings do **not** allow any other operations using the currently 4 existing
[operators](#operators).

### Integers
Integer literals can be created by just typing an integer (currently only supports positive
integer literals, to use a negative integer literal, write `0-#` where # is the absolute value)
```123;```

Integers implement all 4 currently existing [operators](#operators).
```
2 + 1;
>> 3

5 - 1;
>> 4

4 * 4;
>> 16

10 / 2;
>> 5
```

Dividing two integers will **always** result in an **integer**, where everything following the `.`
is **truncated**, meaning even if it's something like `2.99`, the result will be `2`.
```
10 / 4;
>> 2
```

If any side of any operator is a [floating-point number](#floating-point-numbers), the result will
also be a float.
```
2 + 1.5;
>> 3.5

5 - 2.0;
>> 3.0

4.1 * 3;
>> 12.3

10.0 / 4;
>> 2.5
```

*Dividing by zero, whether integer or float, will result in an error.*

### Floating-Point Numbers
Also called floats. Floats can be created by writing a number with decimal places.
```
5.2;
```

If your number is between `0` and `1`, you can leave out the `0` and just write for example `.7`.
```
0.7;
>> 0.7

.7;
>> 0.7
```

**Any** operation using the currently 4 existing [operators](#operators) will result in a float.

*Dividing by zero, whether integer or float, will result in an error.*

### Booleans
Booleans can be created by writing `true` or `false`. Make sure **not** to put these in quotes, as
that will result in a [string literal](#strings) instead.
```
true;
false;
```

Booleans do **not** implement **any** of the currently 4 existing [operators](#operators).

## Identifiers
Identifiers are currently only used to name variables. Identifiers **have** to start with a letter
(uppercase or lowercase), followed by zero or more letters (uppercase or lowercase), numbers and
underscores. Make sure **not** to put these in quotes, as that will result in a
[string literal](#strings) instead.
```
x;
hello_123_world__;
```

## Variables
### Variable declaration
Currently, all variables are mutable. Variables can hold any of the currently 4 existing types
(string, integer, float, boolean). Variables are declared by using the `let` keyword, followed by
at least one whitespace character, following an `identifier` (the name of the variable), followed
by either a semicolon `;`, declaring the end of the statement, or an equals sign `=`, declaring an
immediate initialization.
```
let x;
let y = 5;
```

If the variable is **not** initialized, it has no type. Uninitialized variables can later be
initialized using [delayed variable initialization](#delayed-variable-initialization). If the
variable is initialized upon declaration (also called immediate initialization), the variable will
have the type of the value assigned to it. In the above example, y hold the type `Integer`.

### Delayed Variable Initialization
Variables can be initialized **at** declaration or afterwards. Initializing an already declared
variable is the exact same as [variable assignment](#variable-reassignment), except that you don't
have to worry about using the right type, since the variable doesn't have a type yet.
```
let x;
x = 5;

let y;
y = "Hello";
```

After the variable has been initialized (whether delayed or immediate), it holds the type of the
result of the expression following the `=`, which, in the case of a literal, is just the type of
the literal.

### Variable Reassignment
Also called variable updating. To reassign a variable to a new value, simply use the variable's
identifier without the `let` keyword before it.
```
let x = 5;
x = 10;
```

Note that this **will** require the type of the expression following the `=` to match the type the
variable is holding. If you want to store the value in the variable regardless, look at
[variable shadowing](#variable-shadowing). This means the following is **not** valid:
```
let x = 5;
x = "Hello";
>> Error: TypeMismatch
```

### Variable Shadowing
Also called variable redeclaration. It basically deletes the old variable and overwrites it with
the new one, meaning the variable doesn't hold a type anymore either, letting you choose a new type
for the variable.
```
let x = 5;
let x = "Hello";
```

Note that the old variable will not exist anymore, meaning in the above example, that `5` is lost
forever.

## Operators
### Add `+`
To add two values together, if permitted, write it like this:
```
lhs + rhs;
```

`lsh` and `rhs` are both expressions. If the left type implements addition for the right type, this
will return the result of that. If the left type returns an error for the right type, or it returns
an error for this operator, it will result in an error.
```
"Hello" + 5;
>> Error: IllegalOperation("Cannot add String with non-String type")
```

### Subtract `-`
To subtract a value from another, if permitted, write it like this:
```
lhs - rhs;
```

`lsh` and `rhs` are both expressions. If the left type implements subtraction for the right type,
this will return the result of that. If the left type returns an error for the right type, or it
returns an error for this operator, it will result in an error.
```
"Hello" - "World";
>> Error: IllegalOperation("Subtraction not supported for String type")
```

### Multiply `*`
To multiply two values together, if permitted, write it like this:
```
lhs * rhs;
```

`lsh` and `rhs` are both expressions. If the left type implements multiplication for the right
type, this will return the result of that. If the left type returns an error for the right type, or
it returns an error for this operator, it will result in an error.
```
7.2 * "Something";
>> Error: IllegalOperation("Cannot multiply Float with non-numeric type")
```

### Divide `/`
To divide a value from another, if permitted, write it like this:
```
lhs / rhs;
```

`lsh` and `rhs` are both expressions. If the left type implements division for the right type, this
will return the result of that. If the left type returns an error for the right type, or it returns
an error for this operator, it will result in an error.
```
"Test" / 2;
>> Error: IllegalOperation("Subtraction not supported for String type")
```

Any division by zero, whether float or integer results in a `DivisionByZero` error, unless the
left type doesn't implement division with numbers.
