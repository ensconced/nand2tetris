LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY or16_tb IS
END or16_tb;

ARCHITECTURE Behavioral OF or16_tb IS
  COMPONENT or16 IS
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;

  SIGNAL input_a, input_b, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);

BEGIN
  uut : or16 PORT MAP(
    input_a => input_a,
    input_b => input_b,
    output => output
  );
  stim : PROCESS
    TYPE test_inputs IS ARRAY(2 DOWNTO 0) OF unsigned(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (to_unsigned(0, 16), to_unsigned(1, 16), to_unsigned(1234, 16));
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      FOR j IN test_nums' RANGE LOOP
        input_a <= STD_ULOGIC_VECTOR(test_nums(i));
        input_b <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = (STD_ULOGIC_VECTOR(test_nums(i)) OR STD_ULOGIC_VECTOR(test_nums(j)))) REPORT "test failed for input_a: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_b: " & INTEGER'image(to_integer(test_nums(j)))SEVERITY failure;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;