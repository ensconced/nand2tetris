LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY xor_gate IS
  PORT (
    input_a : IN STD_ULOGIC;
    input_b : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END xor_gate;

ARCHITECTURE structural OF xor_gate IS
  COMPONENT nand_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;

  SIGNAL nand_a_out, nand_b_out, nand_c_out : STD_ULOGIC;
BEGIN
  nand_a : nand_gate PORT MAP(input_a => input_a, input_b => input_b, output => nand_a_out);
  nand_b : nand_gate PORT MAP(input_a => input_a, input_b => nand_a_out, output => nand_b_out);
  nand_c : nand_gate PORT MAP(input_a => nand_a_out, input_b => input_b, output => nand_c_out);
  nand_d : nand_gate PORT MAP(input_a => nand_b_out, input_b => nand_c_out, output => output);
END structural;