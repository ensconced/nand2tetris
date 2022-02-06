LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY and8 IS
  PORT (
    input_a : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
    input_b : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0));
END and8;

ARCHITECTURE structural OF and8 IS
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
BEGIN
  gen_and :
  FOR I IN 0 TO 7 GENERATE
    and_i : and_gate PORT MAP
      (input_a => input_a(i), input_b => input_b(i), output => output(i));
  END GENERATE gen_and;
END structural;