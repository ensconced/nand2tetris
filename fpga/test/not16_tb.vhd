LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

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