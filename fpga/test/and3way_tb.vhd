LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY and3way_tb IS
END and3way_tb;

ARCHITECTURE Behavioral OF and3way_tb IS
  COMPONENT and3way IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL output : STD_ULOGIC;

BEGIN
  uut : and3way PORT MAP(
    input => input,
    output => output
  );

  stim : PROCESS
  BEGIN
    input <= "000";
    WAIT FOR 10 ns;
    ASSERT (output = '0') REPORT "test failed for input 000" SEVERITY failure;
    input <= "001";
    WAIT FOR 10 ns;
    ASSERT (output = '0') REPORT "test failed for input 001" SEVERITY failure;
    input <= "011";
    WAIT FOR 10 ns;
    ASSERT (output = '0') REPORT "test failed for input 011" SEVERITY failure;
    input <= "111";
    WAIT FOR 10 ns;
    ASSERT (output = '1') REPORT "test failed for input 111" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;