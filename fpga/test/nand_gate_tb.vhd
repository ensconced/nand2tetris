LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY nand_gate_tb IS
END nand_gate_tb;

ARCHITECTURE Behavioral OF nand_gate_tb IS
  COMPONENT nand_gate IS
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL a, b, c : STD_ULOGIC;

BEGIN
  uut : nand_gate PORT MAP(
    input_a => a,
    input_b => b,
    output => c
  );

  stim : PROCESS
  BEGIN
    FOR i IN STD_ULOGIC RANGE '0' TO '1' LOOP
      FOR j IN STD_ULOGIC RANGE '0' TO '1' LOOP
        a <= i;
        b <= j;
        WAIT FOR 10 ns;
        ASSERT (c = (a NAND b)) REPORT "test failed for a: "
        & STD_ULOGIC'image(i) &
        "b: " &
        STD_ULOGIC'image(j)
        SEVERITY failure;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;