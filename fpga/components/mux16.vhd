LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux16 IS
  PORT (
    input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    sel : IN STD_ULOGIC;
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
END mux16;

ARCHITECTURE structural OF mux16 IS
  COMPONENT mux
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
BEGIN
  gen_mux :
  FOR I IN 0 TO 15 GENERATE
    mux_i : mux PORT MAP
      (input_a => input_a(i), input_b => input_b(i), sel => sel, output => output(i));
  END GENERATE gen_mux;
END structural;