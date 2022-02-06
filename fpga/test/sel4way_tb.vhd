LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY sel4way_tb IS
END sel4way_tb;

ARCHITECTURE Behavioral OF sel4way_tb IS
  COMPONENT sel4way IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(1 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(3 DOWNTO 0);
BEGIN
  uut : sel4way PORT MAP(
    input => input,
    output => output
  );
  stim : PROCESS
  BEGIN
    input <= "00";
    WAIT FOR 10 ns;
    ASSERT(output = "0001") REPORT "test failed for input 00" SEVERITY failure;
    input <= "01";
    WAIT FOR 10 ns;
    ASSERT(output = "0010") REPORT "test failed for input 01" SEVERITY failure;
    input <= "10";
    WAIT FOR 10 ns;
    ASSERT(output = "0100") REPORT "test failed for input 10" SEVERITY failure;
    input <= "11";
    WAIT FOR 10 ns;
    ASSERT(output = "1000") REPORT "test failed for input 11" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;