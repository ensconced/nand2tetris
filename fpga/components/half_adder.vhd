LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY half_adder IS
  PORT (
    input_a : IN STD_ULOGIC;
    input_b : IN STD_ULOGIC;
    output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
END half_adder;

ARCHITECTURE structural OF half_adder IS
  COMPONENT xor_gate
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
BEGIN
  xor_a : xor_gate PORT MAP(input_a => input_a, input_b => input_b, output => output(0));
  and_a : and_gate PORT MAP(input_a => input_a, input_b => input_b, output => output(1));
END structural;