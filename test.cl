Builtin.println("Hi there!");

class G {
  static void print(string s) { Builtin.print(s); Builtin.print(" "); }
  static void print(bool b) { Builtin.print(b); Builtin.print(" "); }
  static void print(int i) { Builtin.print(i); Builtin.print(" "); }
  static void print(float f) { Builtin.print(f); Builtin.print(" "); }
}

G.print(true);
G.print(1 > 2);
G.print(4 != 4);
G.print(2 + 2 == 4);
Builtin.println("");

G.print(0);
G.print(7);
G.print(10-15);
G.print((5+5)/3);
Builtin.println("");

G.print(0.0);
G.print(3.14);
G.print(1.0/3.0);
G.print(.25*4.0);
Builtin.println("");

G.print(true || false && false);
G.print(!true);
G.print(true && !false);
G.print((true || false) && false);
Builtin.println("");

bool a = false;
bool b = false;

if (a) { G.print("a"); } else if (b) { G.print("b"); }

int i = 0;
int max = 10;

while (i < max) {
  G.print(i);
  i = i + 1;
}

Builtin.println("");

G.print(Builtin.parseBool(""));
G.print(Builtin.parseBool(0));
G.print(Builtin.parseBool(0.0));
G.print(Builtin.parseBool("hello"));
G.print(Builtin.parseBool(3));
G.print(Builtin.parseBool(2.5));

Builtin.println("");

class X {
  /* Field - accessed via .<fieldname> */
  int x;
  static int d = 20;

  /* Constructor - called via .new(<args>) */
  static Self X(int a) { 
    /* TODO: Require all instance-fields to be initialized at the end of constructor. Requires
     * Error on line 121 to be fixed. */
    self.x = a;
    Builtin.println("Constructor!");
  }
  
  static Self X(string s) {
    /* TODO: Require all instance-fields to be initialized at the end of constructor. Requires
     * Error on line 121 to be fixed. */
    self.x = s.toInt();
    Builtin.println("Constructor with string!");
  }

  /* Method - called via .<methodname>(<args>) */
  void printX() {
    Builtin.println("X(" + self.x.toString() + ")");
  }

  static int staticMethod() {
    return 4;
  }

  int _bopAdd(X other) {
    return self.x + other.x;
  }
}

X y = X.new("10");
y.printX();

G.print(y.x);

/* NOTE: Argument type mismatch error works correctly
 * Builtin.println(X.d);
 */

Builtin.println(X.staticMethod().toString());

Builtin.println(Builtin.parseString(y + X.new(5)));

/* NOTE: Missing variable error works correctly
 * Builtin.println(k);
 */

/* NOTE: Missing function error works correctly
 * hehe();
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
 * Builtin.println(somestring);
 */

/* TODO: Lots of missing errors... Fix these, find more later... */
