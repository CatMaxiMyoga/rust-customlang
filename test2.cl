void printHello() {
  Builtin.println("Hello from func");
}

class Main {
  static int main() {
    Builtin.println("Hello, World");
    printHello();
    return 0;
  }
}
