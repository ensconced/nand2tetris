LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux IS
  PORT (
    input : IN STD_ULOGIC;
    sel : IN STD_ULOGIC;
    output_a : OUT STD_ULOGIC;
    output_b : OUT STD_ULOGIC);
END dmux;

ARCHITECTURE structural OF dmux IS
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
  SIGNAL not_out : STD_ULOGIC;
BEGIN
  and_a : and_gate PORT MAP(input_a => not_out, input_b => input, output => output_a);
  and_b : and_gate PORT MAP(input_a => sel, input_b => input, output => output_b);
  not_a : not_gate PORT MAP(input => sel, output => not_out);
END structural;