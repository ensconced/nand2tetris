LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY or_gate IS
  PORT (
    input_a : IN STD_ULOGIC;
    input_b : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END or_gate;

ARCHITECTURE structural OF or_gate IS
  COMPONENT nand_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;

  SIGNAL nand_a_out, nand_b_out : STD_ULOGIC;
BEGIN
  nand_a : nand_gate PORT MAP(input_a => input_a, input_b => input_a, output => nand_a_out);
  nand_b : nand_gate PORT MAP(input_a => input_b, input_b => input_b, output => nand_b_out);
  nand_c : nand_gate PORT MAP(input_a => nand_a_out, input_b => nand_b_out, output => output);
END structural;