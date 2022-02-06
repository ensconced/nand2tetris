LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY sel8way_tb IS
END sel8way_tb;

ARCHITECTURE Behavioral OF sel8way_tb IS
  COMPONENT sel8way IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(7 DOWNTO 0);
BEGIN
  uut : sel8way PORT MAP(
    input => input,
    output => output
  );
  stim : PROCESS
  BEGIN
    input <= "000";
    WAIT FOR 10 ns;
    ASSERT(output = "00000001") REPORT "test failed for input 000" SEVERITY failure;
    input <= "001";
    WAIT FOR 10 ns;
    ASSERT(output = "00000010") REPORT "test failed for input 001" SEVERITY failure;
    input <= "010";
    WAIT FOR 10 ns;
    ASSERT(output = "00000100") REPORT "test failed for input 010" SEVERITY failure;
    input <= "011";
    WAIT FOR 10 ns;
    ASSERT(output = "00001000") REPORT "test failed for input 011" SEVERITY failure;
    input <= "100";
    WAIT FOR 10 ns;
    ASSERT(output = "00010000") REPORT "test failed for input 100" SEVERITY failure;
    input <= "101";
    WAIT FOR 10 ns;
    ASSERT(output = "00100000") REPORT "test failed for input 101" SEVERITY failure;
    input <= "110";
    WAIT FOR 10 ns;
    ASSERT(output = "01000000") REPORT "test failed for input 110" SEVERITY failure;
    input <= "111";
    WAIT FOR 10 ns;
    ASSERT(output = "10000000") REPORT "test failed for input 111" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;