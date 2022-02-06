LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY is_non_zero IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC
  );
END is_non_zero;

ARCHITECTURE structural OF is_non_zero IS
  COMPONENT or8way
    PORT (
      input : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT or_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL or8way_a_out, or8way_b_out : STD_ULOGIC;
BEGIN
  or8way_a : or8way PORT MAP(input => input(15 DOWNTO 8), output => or8way_a_out);
  or8way_b : or8way PORT MAP(input => input(7 DOWNTO 0), output => or8way_b_out);
  or_a : or_gate PORT MAP(input_a => or8way_a_out, input_b => or8way_b_out, output => output);
END structural;