LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY inc16_tb IS
END inc16_tb;

ARCHITECTURE Behavioral OF inc16_tb IS
  COMPONENT inc16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  uut : inc16 PORT MAP(
    input => input,
    output => output
  );

  stim : PROCESS
    TYPE test_inputs IS ARRAY(3 DOWNTO 0) OF unsigned(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (to_unsigned(0, 16), to_unsigned(1, 16), to_unsigned(1234, 16), to_unsigned(65535, 16));
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      input <= STD_ULOGIC_VECTOR(test_nums(i));
      WAIT FOR 10 ns;
      ASSERT (output = (STD_ULOGIC_VECTOR(test_nums(i) + to_unsigned(1, 16)))) REPORT "test failed for input_a: " SEVERITY failure;
    END LOOP;
  END PROCESS;
END Behavioral;