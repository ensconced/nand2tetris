LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY or3way IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC
  );
END or3way;

ARCHITECTURE structural OF or3way IS
  COMPONENT or_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL or_a_out : STD_ULOGIC;
BEGIN
  or_a : or_gate PORT MAP(input_a => input(0), input_b => input(1), output => or_a_out);
  or_b : or_gate PORT MAP(input_a => or_a_out, input_b => input(2), output => output);
END structural;