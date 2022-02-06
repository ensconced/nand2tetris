LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY full_adder_tb IS
END full_adder_tb;

ARCHITECTURE Behavioral OF full_adder_tb IS
  COMPONENT full_adder IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(1 DOWNTO 0);
BEGIN
  uut : full_adder PORT MAP(
    input => input,
    output => output
  );

  stim : PROCESS
  BEGIN
    input <= "000";
    WAIT FOR 10 ns;
    ASSERT (output = "00") REPORT "failed for input: 000" SEVERITY failure;
    input <= "001";
    WAIT FOR 10 ns;
    ASSERT (output = "01") REPORT "failed for input: 001" SEVERITY failure;
    input <= "010";
    WAIT FOR 10 ns;
    ASSERT (output = "01") REPORT "failed for input: 010" SEVERITY failure;
    input <= "011";
    WAIT FOR 10 ns;
    ASSERT (output = "10") REPORT "failed for input: 011" SEVERITY failure;
    input <= "100";
    WAIT FOR 10 ns;
    ASSERT (output = "01") REPORT "failed for input: 100" SEVERITY failure;
    input <= "101";
    WAIT FOR 10 ns;
    ASSERT (output = "10") REPORT "failed for input: 101" SEVERITY failure;
    input <= "110";
    WAIT FOR 10 ns;
    ASSERT (output = "10") REPORT "failed for input: 110" SEVERITY failure;
    input <= "111";
    WAIT FOR 10 ns;
    ASSERT (output = "11") REPORT "failed for input: 111" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;