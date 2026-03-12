using CustomLang.Types;

namespace CustomLang {

public static class rmm_Builtin {
  // ┌──────────┐
  // │ Printing │
  // └──────────┘
  public static void rmm_print(rmm_String s) => System.Console.Write(s.ToString());
  public static void rmm_print(rmm_Bool b) => rmm_print(b.rmm_toString());
  public static void rmm_print(rmm_Int i) => rmm_print(i.rmm_toString());
  public static void rmm_print(rmm_Float f) => rmm_print(f.rmm_toString());

  public static void rmm_println(rmm_String s) => System.Console.WriteLine(s.ToString());
  public static void rmm_println(rmm_Bool b) => rmm_println(b.rmm_toString());
  public static void rmm_println(rmm_Int i) => rmm_println(i.rmm_toString());
  public static void rmm_println(rmm_Float f) => rmm_println(f.rmm_toString());

  // ┌─────────┐
  // │ Parsing │
  // └─────────┘
  public static rmm_String rmm_parseString(rmm_Bool b) => new(b.Inner ? "true" : "false");
  public static rmm_String rmm_parseString(rmm_Int i) => new(i.ToString());
  public static rmm_String rmm_parseString(rmm_Float f) => new(f.ToString());

  public static rmm_Bool rmm_parseBool(rmm_String s) => new(s.Inner != "");
  public static rmm_Bool rmm_parseBool(rmm_Int i) => new(i.Inner != 0);
  public static rmm_Bool rmm_parseBool(rmm_Float f) => new(f.Inner != 0.0);

  public static rmm_Int rmm_parseInt(rmm_String s) => new(int.Parse(s.Inner));
  public static rmm_Int rmm_parseInt(rmm_Bool b) => new(b.Inner ? 1 : 0);
  public static rmm_Int rmm_parseInt(rmm_Float f) => new((int)f.Inner);

  public static rmm_Float rmm_parseFloat(rmm_String s) => new(double.Parse(s.Inner));
  public static rmm_Float rmm_parseFloat(rmm_Bool b) => new(b.Inner ? 1.0 : 0.0);
  public static rmm_Float rmm_parseFloat(rmm_Int i) => new((double)i.Inner);
}

}
