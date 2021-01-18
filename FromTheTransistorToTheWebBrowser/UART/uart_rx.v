module uart_rx(
  rx,
  s_tick,
  dout,
  rx_done_tick,
  reset
);


input rx;
input s_tick;
input reset;

output reg [7:0] dout;
output reg rx_done_tick;

reg [3:0] counter;
reg [2:0] state;
reg [3:0] bit_count;

parameter [5:0]
  IDLE = 5'd0,
  DATA = 5'd1,
  STOP = 5'd2;



always@(posedge s_tick) begin
  case (state)
    IDLE:
      if(rx == 0 && counter == 4'd7) begin
        rx_done_tick <= 0;
        state <= DATA;
        counter <= 0;
        bit_count <= 0;
        dout <= 0;
      end else begin
        counter <= counter + 1;
      end
    DATA:
      if (counter == 4'd15) begin
        state <= DATA;
        dout <= {rx, dout[7:1]};
        bit_count <= bit_count + 1;
        counter <= counter + 1;
        if (bit_count == 3'd7) begin
          state <= STOP;
          bit_count <= 0;
          rx_done_tick <= 0;
        end
      end else begin
        counter <= counter + 1;
      end
      STOP:
        if (counter == 4'd15) begin
          rx_done_tick <= 1;
          state <= IDLE;
          counter <= 0;
        end else begin
          counter <= counter + 1;
        end
      endcase
    end

always@(negedge reset) begin
  state <= IDLE;
  counter <= 4'b0;
  rx_done_tick <= 1;
end


endmodule

  







