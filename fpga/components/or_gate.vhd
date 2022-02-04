entity or_gate is
    port ( input_a : in bit;
           input_b : in bit;
           output : out bit);
end or_gate;

architecture structural of or_gate is
    component nand_gate
      port (
        input_a, input_b : in bit;
        output : out bit
      );
    end component;
    
    signal nand_a_out, nand_b_out : bit;
begin
    nand_a: nand_gate port map (input_a => input_a, input_b => input_a, output => nand_a_out);
    nand_b: nand_gate port map (input_a => input_b, input_b => input_b, output => nand_b_out);
    nand_c: nand_gate port map (input_a => nand_a_out, input_b => nand_b_out, output => output);
end structural;
