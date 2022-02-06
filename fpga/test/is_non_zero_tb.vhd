LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY is_non_zero_tb IS
END is_non_zero_tb;

ARCHITECTURE Behavioral OF is_non_zero_tb IS
  COMPONENT is_non_zero IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC
    );
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL output : STD_ULOGIC;
BEGIN
  uut : is_non_zero PORT MAP(
    input => input,
    output => output
  );

  stim : PROCESS
    TYPE test_inputs IS ARRAY(3 DOWNTO 0) OF unsigned(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (to_unsigned(0, 16), to_unsigned(1, 16), to_unsigned(1234, 16), to_unsigned(65535, 16));
    VARIABLE expected_output : STD_ULOGIC := '0';
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      input <= STD_ULOGIC_VECTOR(test_nums(i));
      WAIT FOR 10 ns;
      IF (test_nums(i) = to_unsigned(0, 16)) THEN
        expected_output := '0';
      ELSE
        expected_output := '1';
      END IF;
      ASSERT (output = expected_output) REPORT "test failed for input: " & INTEGER'image(to_integer(test_nums(i))) SEVERITY failure;
    END LOOP;
  END PROCESS;
END Behavioral;