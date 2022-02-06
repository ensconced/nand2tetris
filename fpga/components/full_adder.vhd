LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY full_adder IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
END full_adder;

ARCHITECTURE structural OF full_adder IS
  COMPONENT half_adder
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
  END COMPONENT;
  COMPONENT or_gate
    PORT (
      input_a, input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL half_adder_a_carry, half_adder_a_sum, half_adder_b_carry : STD_ULOGIC;
BEGIN
  half_adder_a : half_adder PORT MAP(input_a => input(0), input_b => input(1), output(0) => half_adder_a_sum, output(1) => half_adder_a_carry);
  half_adder_b : half_adder PORT MAP(input_a => half_adder_a_sum, input_b => input(2), output(0) => output(0), output(1) => half_adder_b_carry);
  or_a : or_gate PORT MAP(input_a => half_adder_a_carry, input_b => half_adder_b_carry, output => output(1));
END structural;