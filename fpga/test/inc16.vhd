LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY inc16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
END inc16;

ARCHITECTURE structural OF inc16 IS
  COMPONENT add16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  SIGNAL const_one : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  const_one <= STD_ULOGIC_VECTOR(to_unsigned(1, 16));
  adder : add16 PORT MAP(input_a => input, input_b => const_one, output => output);
END structural;