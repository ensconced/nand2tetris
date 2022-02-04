entity nand_gate is
    port ( input_a : in bit;
           input_b : in bit;
           output : out bit);
end nand_gate;

architecture Behavioral of nand_gate is
begin    
    output <= input_a nand input_b;
end Behavioral;

