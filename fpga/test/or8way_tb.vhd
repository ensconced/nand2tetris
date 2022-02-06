LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY or8way_tb IS
END or8way_tb;

ARCHITECTURE Behavioral OF or8way_tb IS
  COMPONENT or8way IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
      output : OUT STD_ULOGIC);
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(7 DOWNTO 0);
  SIGNAL output : STD_ULOGIC;

BEGIN
  uut : or8way PORT MAP(
    input => input,
    output => output
  );
  stim : PROCESS
  BEGIN
    input <= "00000000";
    WAIT FOR 10 ns;
    ASSERT (output = '0') REPORT "test failed with all 0 input" SEVERITY failure;
    input <= "00001000";
    WAIT FOR 10 ns;
    ASSERT (output = '1') REPORT "test failed with input including single 1" SEVERITY failure;
    input <= "11111111";
    WAIT FOR 10 ns;
    ASSERT (output = '1') REPORT "test failed with all 1 input" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;