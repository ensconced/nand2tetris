LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY nand_gate IS
  PORT (
    input_a : IN STD_ULOGIC;
    input_b : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END nand_gate;

ARCHITECTURE Behavioral OF nand_gate IS
BEGIN
  output <= input_a NAND input_b;
END Behavioral;