using CustomLang.Types;

namespace CustomLang {

public static class BuiltinFunctions {
  // ┌──────────┐
  // │ Printing │
  // └──────────┘
  public static void rmm_print(rmm_String s) => System.Console.Write(s.ToString());
  public static void rmm_println(rmm_String s) => System.Console.WriteLine(s.ToString());

  // ┌───────────┐
  // │ *ToString │
  // └───────────┘
  public static rmm_String rmm_boolToString(rmm_Bool b) => new(b.Inner ? "true" : "false");
  public static rmm_String rmm_intToString(rmm_Int i) => new(i.ToString());
  public static rmm_String rmm_floatToString(rmm_Float f) => new(f.ToString());

  // ┌─────────┐
  // │ *ToBool │
  // └─────────┘
  public static rmm_Bool rmm_stringToBool(rmm_String s) => new(s.Inner != "");
  public static rmm_Bool rmm_intToBool(rmm_Int i) => new(i.Inner != 0);
  public static rmm_Bool rmm_floatToBool(rmm_Float f) => new(f.Inner != 0.0);

  // ┌────────┐
  // │ *ToInt │
  // └────────┘
  public static rmm_Int rmm_stringToInt(rmm_String s) => new(int.Parse(s.Inner));
  public static rmm_Int rmm_boolToInt(rmm_Bool b) => new(b.Inner ? 1 : 0);
  public static rmm_Int rmm_floatToInt(rmm_Float f) => new((int)f.Inner);

  // ┌──────────┐
  // │ *ToFloat │
  // └──────────┘
  public static rmm_Float rmm_stringToFloat(rmm_String s) => new(double.Parse(s.Inner));
  public static rmm_Float rmm_boolToFloat(rmm_Bool b) => new(b.Inner ? 1.0 : 0.0);
  public static rmm_Float rmm_intToFloat(rmm_Int i) => new((double)i.Inner);
}

}
