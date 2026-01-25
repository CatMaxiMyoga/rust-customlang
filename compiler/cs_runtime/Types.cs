namespace CustomLang.Types {

public class rmm_String {
  private string value;
  public rmm_String(string value) => this.value = value;

  public override string ToString() => value;
  public string Inner => value;

  public rmm_Bool rmm_toBool() => new(value.Length != 0);
  public rmm_Int rmm_toInt() => new(int.Parse(value));
  public rmm_Float rmm_toFloat() => new(double.Parse(value));

  public rmm_String rmm__bopAdd(rmm_String other) => new(this.value + other.Inner);
  public rmm_Bool rmm__bopEq(rmm_String other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_String other) => new(this.value != other.Inner);
}

public class rmm_Bool {
  private bool value;
  public rmm_Bool(bool value) => this.value = value;

  public override string ToString() => value ? "true" : "false";
  public bool Inner => value;

  public static bool operator true(rmm_Bool b) => b.value;
  public static bool operator false(rmm_Bool b) => !b.value;

  public rmm_String rmm_toString() => new(value ? "true" : "false");
  public rmm_Int rmm_toInt() => new(value ? 1 : 0);
  public rmm_Float rmm_toFloat() => new(value ? 1.0 : 0.0);

  public rmm_Bool rmm__bopEq(rmm_Bool other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_Bool other) => new(this.value != other.Inner);
  public rmm_Bool rmm__bopAnd(rmm_Bool other) => new(this.value && other.Inner);
  public rmm_Bool rmm__bopOr(rmm_Bool other) => new(this.value || other.Inner);

  public rmm_Bool rmm__uopNot() => new(!this.value);
}

public class rmm_Int {
  private int value;
  public rmm_Int(int value) => this.value = value;

  public override string ToString() => value.ToString();
  public int Inner => value;

  public rmm_String rmm_toString() => new(value.ToString());
  public rmm_Bool rmm_toBool() => new(value != 0);
  public rmm_Float rmm_toFloat() => new((double)value);

  public rmm_Int rmm__bopAdd(rmm_Int other) => new(this.value + other.Inner);
  public rmm_Int rmm__bopSub(rmm_Int other) => new(this.value - other.Inner);
  public rmm_Int rmm__bopMul(rmm_Int other) => new(this.value * other.Inner);
  public rmm_Int rmm__bopDiv(rmm_Int other) => new(this.value / other.Inner);
  public rmm_Bool rmm__bopEq(rmm_Int other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_Int other) => new(this.value != other.Inner);
  public rmm_Bool rmm__bopLt(rmm_Int other) => new(this.value < other.Inner);
  public rmm_Bool rmm__bopGt(rmm_Int other) => new(this.value > other.Inner);
  public rmm_Bool rmm__bopLe(rmm_Int other) => new(this.value <= other.Inner);
  public rmm_Bool rmm__bopGe(rmm_Int other) => new(this.value >= other.Inner);

  public rmm_Float rmm__bopAdd(rmm_Float other) => new(this.value + other.Inner);
  public rmm_Float rmm__bopSub(rmm_Float other) => new(this.value - other.Inner);
  public rmm_Float rmm__bopMul(rmm_Float other) => new(this.value * other.Inner);
  public rmm_Float rmm__bopDiv(rmm_Float other) => new(this.value / other.Inner);
  public rmm_Bool rmm__bopEq(rmm_Float other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_Float other) => new(this.value != other.Inner);
  public rmm_Bool rmm__bopLt(rmm_Float other) => new(this.value < other.Inner);
  public rmm_Bool rmm__bopGt(rmm_Float other) => new(this.value > other.Inner);
  public rmm_Bool rmm__bopLe(rmm_Float other) => new(this.value <= other.Inner);
  public rmm_Bool rmm__bopGe(rmm_Float other) => new(this.value >= other.Inner);
}

public class rmm_Float {
  private double value;
  public rmm_Float(double value) => this.value = value;

  public override string ToString() => value.ToString();
  public double Inner => value;

  public rmm_String rmm_toString() => new(value.ToString());
  public rmm_Bool rmm_toBool() => new(value != 0.0);
  public rmm_Int rmm_toInt() => new((int)value);

  public rmm_Float rmm__bopAdd(rmm_Float other) => new(this.value + other.Inner);
  public rmm_Float rmm__bopSub(rmm_Float other) => new(this.value - other.Inner);
  public rmm_Float rmm__bopMul(rmm_Float other) => new(this.value * other.Inner);
  public rmm_Float rmm__bopDiv(rmm_Float other) => new(this.value / other.Inner);
  public rmm_Bool rmm__bopEq(rmm_Float other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_Float other) => new(this.value != other.Inner);
  public rmm_Bool rmm__bopLt(rmm_Float other) => new(this.value < other.Inner);
  public rmm_Bool rmm__bopGt(rmm_Float other) => new(this.value > other.Inner);
  public rmm_Bool rmm__bopLe(rmm_Float other) => new(this.value <= other.Inner);
  public rmm_Bool rmm__bopGe(rmm_Float other) => new(this.value >= other.Inner);

  public rmm_Float rmm__bopAdd(rmm_Int other) => new(this.value + other.Inner);
  public rmm_Float rmm__bopSub(rmm_Int other) => new(this.value - other.Inner);
  public rmm_Float rmm__bopMul(rmm_Int other) => new(this.value * other.Inner);
  public rmm_Float rmm__bopDiv(rmm_Int other) => new(this.value / other.Inner);
  public rmm_Bool rmm__bopEq(rmm_Int other) => new(this.value == other.Inner);
  public rmm_Bool rmm__bopNe(rmm_Int other) => new(this.value != other.Inner);
  public rmm_Bool rmm__bopLt(rmm_Int other) => new(this.value < other.Inner);
  public rmm_Bool rmm__bopGt(rmm_Int other) => new(this.value > other.Inner);
  public rmm_Bool rmm__bopLe(rmm_Int other) => new(this.value <= other.Inner);
  public rmm_Bool rmm__bopGe(rmm_Int other) => new(this.value >= other.Inner);
}

}
