LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY not_gate IS
  PORT (
    input : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END not_gate;

ARCHITECTURE structural OF not_gate IS
  COMPONENT nand_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
BEGIN
  nand_a : nand_gate PORT MAP(input_a => input, input_b => input, output => output);
END structural;