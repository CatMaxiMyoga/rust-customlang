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

  /* Constructor - called via .new(<args>) */
  static Self X(int a) { 
    x = a;
  }

  /* Method - called via .<methodname>(<args>) */
  void printX() {
    println("X(" + intToString(self.x) + ")");
  }
}

X y = X.new(10);
y.printX();
printInt(y.x);
println("");
