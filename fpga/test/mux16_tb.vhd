LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY mux16_tb IS
END mux16_tb;

ARCHITECTURE Behavioral OF mux16_tb IS
  COMPONENT mux16 IS
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;

  SIGNAL input_a, input_b, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL sel : STD_ULOGIC;

BEGIN
  uut : mux16 PORT MAP(
    input_a => input_a,
    input_b => input_b,
    sel => sel,
    output => output
  );
  stim : PROCESS
    TYPE test_inputs IS ARRAY(2 DOWNTO 0) OF unsigned(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (to_unsigned(0, 16), to_unsigned(1, 16), to_unsigned(1234, 16));
    VARIABLE val_for_input_a : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    VARIABLE val_for_input_b : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    VARIABLE val_for_sel : STD_ULOGIC := '0';
    VARIABLE expected_value : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_unsigned(0, 16));
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      val_for_input_a := STD_ULOGIC_VECTOR(test_nums(i));
      FOR j IN test_nums' RANGE LOOP
        val_for_input_b := STD_ULOGIC_VECTOR(test_nums(j));
        FOR k IN 0 TO 1 LOOP
          input_a <= val_for_input_a;
          input_b <= val_for_input_b;
          IF (k = 0) THEN
            expected_value := val_for_input_a;
            val_for_sel := '0';
          ELSE
            expected_value := val_for_input_b;
            val_for_sel := '1';
          END IF;
          sel <= val_for_sel;
          WAIT FOR 10 ns;
          ASSERT (output = expected_value) REPORT "test failed for input_a: " &
          INTEGER'image(to_integer(test_nums(i))) & " input_b: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        END LOOP;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;