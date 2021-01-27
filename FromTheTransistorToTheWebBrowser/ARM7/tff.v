// Toogle Flip Flop

module tff(q, clk, reset);

  input clk, reset;
  output reg q;

  always @(posedge reset or posedge clk) begin
    if (reset) begin
      q <= 1'b0;
    end else begin
      q <= ~q;
    end
  end
endmodule


// Ripple Carry Counter


module ripple_carry_counter (q, clk, reset);

  output [3:0] q;
  input clk, reset;

  tff tff0(q[0], ~clk, reset);
  tff tff1(q[0], ~q[0], reset);
  tff tff2(q[0], ~q[1], reset);
  tff tff3(q[0], ~q[2], reset);

endmodule
