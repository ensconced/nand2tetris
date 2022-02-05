LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux IS
  PORT (
    input_a : IN STD_ULOGIC;
    input_b : IN STD_ULOGIC;
    sel : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END mux;

ARCHITECTURE structural OF mux IS
  COMPONENT or_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT and_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL not_out, and_a_out, and_b_out : STD_ULOGIC;
BEGIN
  and_a : and_gate PORT MAP(input_a => input_a, input_b => not_out, output => and_a_out);
  and_b : and_gate PORT MAP(input_a => input_b, input_b => sel, output => and_b_out);
  not_a : not_gate PORT MAP(input => sel, output => not_out);
  or_a : or_gate PORT MAP(input_a => and_a_out, input_b => and_b_out, output => output);
END structural;