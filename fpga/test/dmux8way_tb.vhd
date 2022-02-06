LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux8way_tb IS
END dmux8way_tb;

ARCHITECTURE Behavioral OF dmux8way_tb IS
  COMPONENT dmux8way IS
    PORT (
      input : IN STD_ULOGIC;
      sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC;
  SIGNAL sel : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(7 DOWNTO 0);
BEGIN
  uut : dmux8way PORT MAP(
    input => input,
    sel => sel,
    output => output
  );

  stim : PROCESS
  BEGIN
    input <= '0';
    sel <= "000";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 000" SEVERITY failure;
    sel <= "001";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 001" SEVERITY failure;
    sel <= "010";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 010" SEVERITY failure;
    sel <= "011";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 011" SEVERITY failure;
    sel <= "100";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 100" SEVERITY failure;
    sel <= "101";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 101" SEVERITY failure;
    sel <= "110";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 110" SEVERITY failure;
    sel <= "111";
    WAIT FOR 10 ns;
    ASSERT (output = "00000000") REPORT "failed with input: 0, sel: 111" SEVERITY failure;

    input <= '1';
    sel <= "000";
    WAIT FOR 10 ns;
    ASSERT (output = "00000001") REPORT "failed with input: 1, sel: 000" SEVERITY failure;
    sel <= "001";
    WAIT FOR 10 ns;
    ASSERT (output = "00000010") REPORT "failed with input: 1, sel: 001" SEVERITY failure;
    sel <= "010";
    WAIT FOR 10 ns;
    ASSERT (output = "00000100") REPORT "failed with input: 1, sel: 010" SEVERITY failure;
    sel <= "011";
    WAIT FOR 10 ns;
    ASSERT (output = "00001000") REPORT "failed with input: 1, sel: 011" SEVERITY failure;
    sel <= "100";
    WAIT FOR 10 ns;
    ASSERT (output = "00010000") REPORT "failed with input: 1, sel: 100" SEVERITY failure;
    sel <= "101";
    WAIT FOR 10 ns;
    ASSERT (output = "00100000") REPORT "failed with input: 1, sel: 101" SEVERITY failure;
    sel <= "110";
    WAIT FOR 10 ns;
    ASSERT (output = "01000000") REPORT "failed with input: 1, sel: 110" SEVERITY failure;
    sel <= "111";
    WAIT FOR 10 ns;
    ASSERT (output = "10000000") REPORT "failed with input: 1, sel: 111" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;