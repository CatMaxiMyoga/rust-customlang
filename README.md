**Updated last: 06/12/2025** *(Up-To-Date)*

I'm making my own custom lexer, parser and compiler in Rust. These are the currently working
language features:

# Table of Contents
- [Currently Supported Literals](#currently-supported-literals)
    - [Strings](#strings)
    - [Integers](#integers)
    - [Floating-Point Numbers (floats)](#floating-point-numbers)
    - [Booleans](#booleans)
- [Identifiers](#identifiers)
- [Variables](#variables)
    - [Variable Declaration](#variable-declaration)
    - [Delayed Variable Initialization](#delayed-variable-initialization)
    - [Variable Reassignment](#variable-reassignment)
    - [Variable Shadowing](#variable-shadowing)
- [Types](#types)
- [Functions](#functions)
    - [Function Declaration](#function-declaration)
    - [Function Call](#function-call)
    - [Builtin Functions](#builtin-functions)
        - [`print(String)`](#builtin-print)
        - [`println(String)`](#builtin-println)
        - [`boolToString(Bool)`](#builtin-tostring)
        - [`intToString(Int)`](#builtin-tostring)
        - [`floatToString(Float)`](#builtin-tostring)
        - [`stringToBool(String)`](#builtin-tobool)
        - [`intToBool(Int)`](#builtin-tobool)
        - [`floatToBool(Float)`](#builtin-tobool)
        - [`stringToInt(String)`](#builtin-toint)
        - [`boolToInt(Bool)`](#builtin-toint)
        - [`floatToInt(Float)`](#builtin-toint)
        - [`stringToFloat(String)`](#builtin-tofloat)
        - [`boolToFloat(Bool)`](#builtin-tofloat)
        - [`intToFloat(Int)`](#builtin-tofloat)
- [Operators](#operators)
    - [Binar Operators](#binary-operators)
        - [Operator Precedence](#operator-precedence)
        - [Additive Operators](#additive-operators)
            - [Add `+`](#add-)
            - [Subtract `-`](#subtract--)
        - [Multiplicative Operators](#multiplicative-operators)
            - [Multiply `*`](#multiply-)
            - [Divide `/`](#divide-)
        - [Comparison Operators](#comparison-operators)
            - [Equal `==`](#equals-comparison-)
            - [Not Equal `!=`](#not-equals-comparison-)
            - [Less Than `<`](#less-than-comparison-)
            - [Greater Than `>`](#greater-than-comparison-)
            - [Less Than or Equal To `<=`](#less-than-or-equal-to-comparison-)
            - [Greater Than or Equal To `>=`](#greater-than-or-equal-to-comparison-)
    - [Unary Operators](#unary-operators)
        - [Not `!`](#not-)
- [If Statements](#if-statements)
- [While Loops](#while-loops)

# Currently Supported Literals
## Strings
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

## Integers
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

## Floating-Point Numbers
Also called floats. Floats can be created by writing a number with decimal places. (currently only
supports positive float literals, to use a negative float literal, write `0-#` where # is the
absolute value)

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

Note that simply doing `5.` is **not** allowed and in fact **invalid**. instead, do `5.0`.

*Dividing by zero, whether integer or float, will result in an error.*

## Booleans
Booleans can be created by writing `true` or `false`. Make sure **not** to put these in quotes, as
that will result in a [string literal](#strings) instead.
```
true;
false;
```

Booleans do **not** implement **any** of the currently 4 existing [operators](#operators).

# Identifiers
Identifiers are currently only used to name variables. Identifiers **have** to start with a letter
(uppercase or lowercase), followed by zero or more letters (uppercase or lowercase), numbers and
underscores. Make sure **not** to put these in quotes, as that will result in a
[string literal](#strings) instead.
```
x;
hello_123_world__;
```

# Variables
## Variable Declaration
Currently, all variables are mutable. Variables can hold any of the currently 4 existing types
(string, integer, float, boolean). Variables are declared by specifying the [type](#types),
followed by at least one whitespace character, following an [identifier](#identifiers) (the name
of the variable), followed by either a semicolon `;`, declaring the end of the statement, or an
equals sign `=`, declaring an immediate initialization, followed by an expression (the value).
```
Int x;
Int y = 5;
```

If the variable is **not** initialized, it has no type. Uninitialized variables can later be
initialized using [delayed variable initialization](#delayed-variable-initialization). If the
variable is initialized upon declaration (also called immediate initialization), the variable will
have the type of the value assigned to it. In the above example, y holds the type `Int`.

## Delayed Variable Initialization
Variables can be initialized **at** declaration or afterwards. Initializing an already declared
variable is the exact same as [variable reassignment](#variable-reassignment).
```
Int x;
x = 5;

String y;
y = "Hello";
```

*Identifiers holding functions **cannot** be used as variable names, as function variables are not
allowed to be overwritten!*

## Variable Reassignment
Also called variable updating. To reassign a variable to a new value, simply use the variable's
identifier without the [type](#types) before it.
```
Int x = 5;
x = 10;
```

Note that this **will** require the type of the expression following the `=` to match the type the
variable is holding. If you want to store the value in the variable regardless, look at
[variable shadowing](#variable-shadowing). This means the following is **not** valid:
```
Int x = 5;
x = "Hello";
>> Error: TypeMismatch
```

*Identifiers holding functions **cannot** be reassigned, as function variables are not allowed to
be overwritten!*

## Variable Shadowing
Also called variable redeclaration. It basically deletes the old variable and overwrites it with
the new one, meaning the variable doesn't hold a type anymore either, letting you choose a new type
for the variable.
```
Int x = 5;
String x = "Hello";
```

Note that the old variable will not exist anymore, meaning in the above example `5` is lost
forever.

*Identifiers holding functions **cannot** be used as variable names, as function variables are not
allowed to be overwritten!*

# Types
Types are [identifiers](#identifiers) that represent the type a variable holds, the type a function
returns or the type a parameter has to be. Types are put before `identifiers` that represent the
name of the function or variable. There's currently 5 valid types.

- `Int` (See [Integers](#integers))
- `Float` (See [Floating-Point Numbers](#floating-point-numbers))
- `String` (See [Strings](#strings))
- `Bool` (See [Booleans](#booleans))
- `Void`

`Void` is a special type that is **only** allowed as a type for [functions](#functions). Using
`Void` as a function's return type specifies that there are no return statements in that function.

# Functions
## Function Declaration
Functions can be declared using the [type](#types), followed by an [identifier](#identifiers), then
parentheses holding parameter `types` and `names` seperated by commas `,`, or just empty
parentheses `()` if the function takes no parameters, and then braces containing a code block. Note
that a function declaration does **not** end with a semicolon.
```
Void hello() { print("Hello"); }
Int square(Int x) { return x * x; }
Float divide(Float a, Float b) { return a / b; }
```

The `return` keyword can **not** be used in the global scope, meaning you can only use it inside
functions. If the function has no `return`, the return type has to be `Void`. This type is not
allowed to be assigned to variables or operated on.

*Note that the function cannot use an identifier that's already been used as a variable, no matter
what type that variable may hold.*

## Function Call
Calling functions is done by specifying the [identifier](#identifiers) holding the function and
putting parentheses behind it. If the function takes no arguments, you can just use empty
parentheses `()`, otherwise the arguments for the function can be specified using comma-seperated
`expressions` (Identifiers, literals, operations like `5 + x`). Note that the amount of arguments
**must** match the parameters the function takes **exactly**.
```
Int square(Int x) { return x * x; }
square(5);

Int n = 10;
square(n);
```

When a function is called, it gets its own scope. The function's inner code block has access to
functions from the parent scope, but it is not allowed to access non-function variables from the
outer scope. Trying that will simply result in a [VariableNotFound](#variable-not-found-error)
error, as the variable is inaccessible, unless assigned inside the function scope, or a function in
the parent's scope.

If you have a function within a function, that inner function can still access functions from the
global scope, as searching the parent environment for functions is recursive until it is found.

If your inner function looks for `x`, and `x` is a function in the global or any parent scope, it
will have access to `x`. However, if we have an inner function, `x` is a function in the global
scope, but the outer function overwrites `x` as a non-function variable, `x` will be found inside
the outer function's scope and the search will stop and return a
[VariableNotFound](#variable-not-found-error) error, since the `x` that was found is not a
function.

## Builtin Functions
Builtin functions are functions that the interpreter adds to the environment by default. They
execute rust functions in the interpreter.

#### Builtin `print`
The print function would be defined as

```
Void print(String c) { ... }
```

It executes `print!("{c}")`

#### Builtin `println`
The println function would be defined as

```
Void println(String c) { ... }
```

It works the same as the [print](#builtin-print) function, but calls `println!("{c}")` instead.

#### Builtin `*ToString`
These functions turn other value types into [String](#strings) types so they can be printed. These
functions would be defined as
```
String boolToString(Bool b) { ... }
String intToString(Int i) { ... }
String floatToString(Float f) { ... }
```

#### Builtin `*ToBool`
These functions turn other value types into [Bool](#booleans) types so they can be used in
conditions. These functions would be defined as
```
Bool stringToBool(String s) { ... }
Bool intToBool(Int i) { ... }
Bool floatToBool(Float f) { ... }
```

#### Builtin `*ToInt`
These functions turn other value types into [Int](#integers) types. These functions would be
defined as
```
Int stringToInt(String s) { ... }
Int boolToInt(Bool b) { ... }
Int floatToInt(Float f) { ... }
```

#### Builtin `*ToFloat`
These functions turn other value types into [Float](#floating-point-numbers) types. These functions
would be defined as
```
Float stringToFloat(String s) { ... }
Float boolToFloat(Bool b) { ... }
Float intToFloat(Int i) { ... }
```

# Operators
## Binary Operators
### Operator Precedence
Of course, this language respects operator precedence, meaning
[multiplicative operations](#multiplicative-operators) before
[additive operations](#additive-operators), [comparison operations](#comparison-operators) after
all and parentheses before all.
```
5 + 5 * 2;
>> 15

(5 + 5) * 2;
>> 20

5 + 5 > 2 * 4;
>> true
```

### Additive Operators
#### Add `+`
To add two values together, if permitted, write it like this:

```
lhs + rhs;
```

`lhs` and `rhs` are both expressions. If the left type implements addition for the right type, this
will return the result of that. If the left type returns an error for the right type, or it returns
an error for this operator, it will result in an error.
```
"Hello" + 5;
>> Error: IllegalOperation("Cannot add String with non-String type")
```

#### Subtract `-`
To subtract a value from another, if permitted, write it like this:

```
lhs - rhs;
```

`lhs` and `rhs` are both expressions. If the left type implements subtraction for the right type,
this will return the result of that. If the left type returns an error for the right type, or it
returns an error for this operator, it will result in an error.
```
"Hello" - "World";
>> Error: IllegalOperation("Subtraction not supported for String type")
```

### Multiplicative Operators
#### Multiply `*`
To multiply two values together, if permitted, write it like this:

```
lhs * rhs;
```

`lhs` and `rhs` are both expressions. If the left type implements multiplication for the right
type, this will return the result of that. If the left type returns an error for the right type, or
it returns an error for this operator, it will result in an error.
```
7.2 * "Something";
>> Error: IllegalOperation("Cannot multiply Float with non-numeric type")
```

#### Divide `/`
To divide a value by another, if permitted, write it like this:

```
lhs / rhs;
```

`lhs` and `rhs` are both expressions. If the left type implements division for the right type, this
will return the result of that. If the left type returns an error for the right type, or it returns
an error for this operator, it will result in an error.
```
"Test" / 2;
>> Error: IllegalOperation("Division not supported for String type")
```

Any division by zero, whether float or integer results in a `DivisionByZero` error, unless the
left type doesn't implement division with numbers.
```
10 / 0;
>> Error: DivisionByZero
```

### Comparison Operators
#### Equals Comparison `==`
Tests if two values **are** the same.
```
"Test" == "Test";
>> true

"Hello" == "World";
>> false

5 == 5;
>> true
```

#### Not Equals Comparison `!=`
Tests if two values are **not** the same.
```
"Hello" != "World";
>> true

5 != 5;
>> false

1.5 != 2.0;
>> true
```

#### Less-Than Comparison `<`
Tests if `lhs` is less than `rhs`.
```
lhs < rhs

5 < 10;
>> true

215 < 215;
>> false

2.0 < 1.5;
>> false
```

#### Greater-Than Comparison `>`
Tests if `lhs` is greater than `rhs`.
```
lhs > rhs

5 > 10;
>> false

215 > 215;
>> false

2.0 > 1.5;
>> true
```

#### Less-Than Or Equal To Comparison `<=`
Tests if `lhs` is either less than `rhs` or equal to it.
```
lhs <= rhs

5 <= 10;
>> true

215 <= 215;
>> true

2.0 <= 1.5;
>> false
```

#### Greater-Than Or Equal To Comparison `>=`
Tests if `lhs` is either greater then `rhs` or equal to it.
```
lhs >= rhs

5 >= 10;
>> false

215 >= 215;
>> true

2.0 >= 1.5;
>> true
```

### Logical Operators
#### Or `||`
Tests if either side is or evaluates to `true`
```
5 > 2 || false
>> true

false || 10 <= 3
>> false

5 > 2 || 10 >= 10
>> true
```

#### And `&&`
Tests if both sides are or evaluate to `true`
```
5 > 2 && false
>> false

false && 10 <= 3
>> false

5 > 2 && 10 >= 10
>> true
```

## Unary Operators
### Not `!`
Switches the value of the boolean expression following it.
```
!true
>> false

!false
>> true

!(5 > 2)
>> false
```

# If Statements
If statements are written using the `if` keyword followed by parentheses containing the condition
evaluating to a [boolean](#booleans) followed by braces `{}` containing the code to run if the
condition evaluates to `true`.
```
if (condition) {
    println("If");
}
```

After the closing brace `}` of the code block you can add an `else` statement, followed by either
braces `{}` containing the code to run if none of the `ìf` or `else-if` statements' conditions have
evaluated to `true`, or another `if` statement, making it an `else-if` statement, meaning if the
previous `if` or `else-if` statements' condition evaluated to false, this `else-if` statements'
condition is evaluated. If the previous statements' condition evaluated to true, this statements'
condition is never evaluated and the code never run.
```
if (condition) {
    println("1");
} else if (condition2) {
    println("2");
} else if (condition3) {
    println("3");
} else {
    println("else");
}
```

# While loops
While loops are written using the `while` keyword followed by parentheses containing the condition
evaluating to a [boolean](#booleans) followed by braces `{}` containing the code to run while the
condition evaluates to `true`.
```
while (condition) {
    println("true");
}
```

If you want to run a loop a specific amount of times, you can make a variable that is the counter
and either hardcode the end:
```
Int i = 0;
while (i < 10) {
    println(intToString(i)+" ");
    i = i + 1;
}
>> 0 1 2 3 4 5 6 7 8 9 
```

or use another variable:
```
Int i = 0;
Int max = 10;
while (i < max) {
    println(intToString(i)+" ");
    i = i + 1;
}
>> 0 1 2 3 4 5 6 7 8 9 
```

In both of these cases the loop runs 10 times, starting at 0 and stopping when `i` becomes 10. It's
important that the variable used as the counter, in this case `i`, is changed inside the while
loop, otherwise it will run forever.
