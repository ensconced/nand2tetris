LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY or16 IS
  PORT (
    input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
END or16;

ARCHITECTURE structural OF or16 IS
  COMPONENT or_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
BEGIN
  gen_or :
  FOR I IN 0 TO 15 GENERATE
    or_i : or_gate PORT MAP
      (input_a => input_a(i), input_b => input_b(i), output => output(i));
  END GENERATE gen_or;
END structural;