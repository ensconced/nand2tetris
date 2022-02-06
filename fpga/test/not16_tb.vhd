LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY not16_tb IS
END not16_tb;

ARCHITECTURE Behavioral OF not16_tb IS
  COMPONENT not16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;

  SIGNAL input, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);

BEGIN
  uut : not16 PORT MAP(
    input => input,
    output => output
  );
  stim : PROCESS
    TYPE test_inputs IS ARRAY(2 DOWNTO 0) OF unsigned(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (to_unsigned(0, 16), to_unsigned(1, 16), to_unsigned(1234, 16));
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      input <= STD_ULOGIC_VECTOR(test_nums(i));
      WAIT FOR 10 ns;
      ASSERT (output = (NOT STD_ULOGIC_VECTOR(test_nums(i)))) REPORT "test failed for input: " & INTEGER'image(to_integer(test_nums(i))) SEVERITY failure;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;