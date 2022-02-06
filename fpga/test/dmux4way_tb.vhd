LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux4way_tb IS
END dmux4way_tb;

ARCHITECTURE Behavioral OF dmux4way_tb IS
  COMPONENT dmux4way IS
    PORT (
      input : IN STD_ULOGIC;
      sel : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC;
  SIGNAL sel : STD_ULOGIC_VECTOR(1 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(3 DOWNTO 0);
BEGIN
  uut : dmux4way PORT MAP(
    input => input,
    sel => sel,
    output => output
  );

  stim : PROCESS
  BEGIN
    input <= '0';
    sel <= "00";
    WAIT FOR 10 ns;
    ASSERT (output = "0000") REPORT "failed with input: 0, sel: 00" SEVERITY failure;
    sel <= "01";
    WAIT FOR 10 ns;
    ASSERT (output = "0000") REPORT "failed with input: 0, sel: 01" SEVERITY failure;
    sel <= "10";
    WAIT FOR 10 ns;
    ASSERT (output = "0000") REPORT "failed with input: 0, sel: 10" SEVERITY failure;
    sel <= "11";
    WAIT FOR 10 ns;
    ASSERT (output = "0000") REPORT "failed with input: 0, sel: 11" SEVERITY failure;
    input <= '1';
    sel <= "00";
    WAIT FOR 10 ns;
    ASSERT (output = "0001") REPORT "failed with input: 1, sel: 00" SEVERITY failure;
    sel <= "01";
    WAIT FOR 10 ns;
    ASSERT (output = "0010") REPORT "failed with input: 1, sel: 01" SEVERITY failure;
    sel <= "10";
    WAIT FOR 10 ns;
    ASSERT (output = "0100") REPORT "failed with input: 1, sel: 10" SEVERITY failure;
    sel <= "11";
    WAIT FOR 10 ns;
    ASSERT (output = "1000") REPORT "failed with input: 1, sel: 11" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;