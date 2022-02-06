LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY and3way IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC);
END and3way;

ARCHITECTURE structural OF and3way IS
  COMPONENT and_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;

  SIGNAL and_b_out : STD_ULOGIC;
BEGIN
  and_a : and_gate PORT MAP(input_a => and_b_out, input_b => input(2), output => output);
  and_b : and_gate PORT MAP(input_a => input(0), input_b => input(1), output => and_b_out);
END structural;