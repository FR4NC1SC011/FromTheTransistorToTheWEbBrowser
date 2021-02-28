package com.craftinginterpreters.lox;

class Return extends RuntimeException {
  final Object value;

  Return(Object value) {
    super(null, null, false, false);
    this.value = value;
  }


  @Override
  public Void visitSetExpr (Expr.Set expr) {
    resolve(expr.value);
    resolve(expr.object);
    return null;
  }
}
