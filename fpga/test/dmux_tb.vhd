LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux_tb IS
END dmux_tb;

ARCHITECTURE Behavioral OF dmux_tb IS
  COMPONENT dmux IS
    PORT (
      input : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output_a : OUT STD_ULOGIC;
      output_b : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, sel, out_a, out_b : STD_ULOGIC;

BEGIN
  uut : dmux PORT MAP(
    input => input,
    sel => sel,
    output_a => out_a,
    output_b => out_b
  );

  stim : PROCESS
    VARIABLE expected_output_a : STD_ULOGIC := '0';
    VARIABLE expected_output_b : STD_ULOGIC := '0';
  BEGIN
    FOR i IN STD_ULOGIC RANGE '0' TO '1' LOOP
      FOR j IN STD_ULOGIC RANGE '0' TO '1' LOOP
        input <= i;
        sel <= j;
        IF (j = '0') THEN
          expected_output_a := i;
          expected_output_b := '0';
        ELSE
          expected_output_b := i;
          expected_output_a := '0';
        END IF;
        WAIT FOR 10 ns;
        ASSERT (out_a = expected_output_a) REPORT "test failed for output_a. input: " &
        STD_ULOGIC'image(i) &
        " sel: " &
        STD_ULOGIC'image(j)
        SEVERITY failure;
        ASSERT (out_b = expected_output_b) REPORT "test failed for output_b. input: " &
        STD_ULOGIC'image(i) &
        " sel: " &
        STD_ULOGIC'image(j)
        SEVERITY failure;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;