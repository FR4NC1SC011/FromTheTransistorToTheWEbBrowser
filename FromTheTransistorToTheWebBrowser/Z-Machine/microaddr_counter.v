module microaddr_counter (
input logic clk,
input logic reset,
input microaddr::cmd cmd,
input logic[10:0] load_addr,

output logic[10:0] addr
);

logic[10:0] next_addr;

always_comb begin
	unique case(cmd)
end

