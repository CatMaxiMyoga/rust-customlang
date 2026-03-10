println("Hi there!");

void printBool(bool b) {
  print(boolToString(b) + " ");
}

printBool(true);
printBool(1 > 2);
printBool(4 != 4);
printBool(2 + 2 == 4);
println("");

void printInt(int i) {
  print(intToString(i) + " ");
}

printInt(0);
printInt(7);
printInt(10-15);
printInt((5+5)/3);
println("");

void printFloat(float f) {
  print(floatToString(f) + " ");
}

printFloat(0.0);
printFloat(3.14);
printFloat(1.0/3.0);
printFloat(.25*4.0);
println("");

printBool(true || false && false);
printBool(!true);
printBool(true && !false);
printBool((true || false) && false);
println("");

bool a = false;
bool b = false;

if (a) { print("a"); } else if (b) { print("b"); }

int i = 0;
int max = 10;

while (i < max) {
  printInt(i);
  i = i + 1;
}

println("");

printBool(stringToBool(""));
printBool(intToBool(0));
printBool(floatToBool(0.0));
printBool(stringToBool("hello"));
printBool(intToBool(3));
printBool(floatToBool(2.5));

println("");

class X {
  /* Field - accessed via .<fieldname> */
  int x;
  static int d = 20;

  /* Constructor - called via .new(<args>) */
  static Self X(int a) { 
    self.x = a;
    println("Constructor!");
  }

  /* Method - called via .<methodname>(<args>) */
  void printX() {
    println("X(" + self.x.toString() + ")");
  }

  static int staticMethod() {
    return 4;
  }
}

X y = X.new(10);
y.printX();

printInt(y.x);

/* FIXME: Doesn't throw semantic error even though it's an int where a string is expected
 * println(X.d);
 */

println(X.staticMethod().toString());

/* FIXME: Doesn't throw semantic error even though it's an int where a string is expected
 * println(1);
 */

void strfunc(string s) {
  println(s);
}

/* FIXME: Doesn't throw semantic error even though it's an int where a string is expected
 * strfunc(2);
 */

/* NOTE: Missing variable error works correctly
 * println(k);
 */

/* NOTE: Missing function error works correctly
 * hehe();
 */

/* FIXME: Allows making variables of type void
 * void x = X.printX();
 */

/* FIXME: Not all code paths have to return a value in a non-void function or method
 * string s(bool b) {
 *   if (b) {
 *     return "true";
 *   }
 * }
 */

/* FIXME: Not all code paths have to initialize a variable before it's used
 * bool somebool = true;
 * string somestring;
 * if (somebool) {
 *   somestring = "hello";
 * }
 * println(somestring);
 */

/* TODO: Lots of missing errors... Fix these, find more later... */
