LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY sel4way IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0)
  );
END sel4way;

ARCHITECTURE structural OF sel4way IS
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL not_a_out, not_b_out : STD_ULOGIC;
BEGIN
  not_a : not_gate PORT MAP(input => input(0), output => not_a_out);
  not_b : not_gate PORT MAP(input => input(1), output => not_b_out);
  and_a : and_gate PORT MAP(input_a => not_a_out, input_b => not_b_out, output => output(0));
  and_b : and_gate PORT MAP(input_a => input(0), input_b => not_b_out, output => output(1));
  and_c : and_gate PORT MAP(input_a => not_a_out, input_b => input(1), output => output(2));
  and_d : and_gate PORT MAP(input_a => input(0), input_b => input(1), output => output(3));
END structural;