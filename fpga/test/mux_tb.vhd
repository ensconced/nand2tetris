LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux_tb IS
END mux_tb;

ARCHITECTURE Behavioral OF mux_tb IS
  COMPONENT mux IS
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL a, b, sel, output : STD_ULOGIC;

BEGIN
  uut : mux PORT MAP(
    input_a => a,
    input_b => b,
    sel => sel,
    output => output
  );

  stim : PROCESS
    VARIABLE expected_output : STD_ULOGIC := '0';
  BEGIN
    FOR i IN STD_ULOGIC RANGE '0' TO '1' LOOP
      FOR j IN STD_ULOGIC RANGE '0' TO '1' LOOP
        FOR k IN STD_ULOGIC RANGE '0' TO '1' LOOP
          a <= i;
          b <= j;
          sel <= k;
          IF (k = '1') THEN
            expected_output := j;
          ELSE
            expected_output := i;
          END IF;
          WAIT FOR 100 ns;
          ASSERT (output = expected_output) REPORT "test failed for a: " &
          STD_ULOGIC'image(i) &
          " b: " &
          STD_ULOGIC'image(j) &
          " sel: " &
          STD_ULOGIC'image(k)
          SEVERITY failure;
        END LOOP;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;