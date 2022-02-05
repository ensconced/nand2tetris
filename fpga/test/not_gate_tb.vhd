LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY not_gate_tb IS
END not_gate_tb;

ARCHITECTURE Behavioral OF not_gate_tb IS
  COMPONENT not_gate IS
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL a, b : STD_ULOGIC;

BEGIN
  uut : not_gate PORT MAP(
    input => a,
    output => b
  );

  stim : PROCESS
  BEGIN
    FOR i IN STD_ULOGIC RANGE '0' TO '1' LOOP
      a <= i;
      WAIT FOR 10 ns;
      ASSERT (b = (NOT a)) REPORT "test failed for a: "
      & STD_ULOGIC'image(i)
      SEVERITY failure;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;