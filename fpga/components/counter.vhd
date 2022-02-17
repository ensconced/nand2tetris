LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY counter IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END counter;

ARCHITECTURE structural OF counter IS
  COMPONENT mux
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
BEGIN
  gen_reg :
  FOR I IN 0 TO 15 GENERATE
    reg_i : bit_register PORT MAP
      (input => input(i), output => output(i), load => load, clock => clock);
  END GENERATE gen_reg;
END structural;