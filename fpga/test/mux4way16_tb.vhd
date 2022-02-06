LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux4way16_tb IS
END mux4way16_tb;

ARCHITECTURE Behavioral OF mux4way16_tb IS
  COMPONENT mux4way16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(63 DOWNTO 0);
      sel : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;

  SIGNAL input_a, input_b, input_c, input_d : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL sel : STD_ULOGIC_VECTOR(1 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  uut : mux4way16 PORT MAP(
    input(63 DOWNTO 48) => input_d,
    input(47 DOWNTO 32) => input_c,
    input(31 DOWNTO 16) => input_b,
    input(15 DOWNTO 0) => input_a,
    sel => sel,
    output => output
  );
  stim : PROCESS
  BEGIN
    input_a <= "0000000000000000";
    input_b <= "1010101010101010";
    input_c <= "0101010101010101";
    input_d <= "1111111111111111";
    sel <= "00";
    WAIT FOR 10 ns;
    ASSERT(output = "0000000000000000") REPORT "test failed for sel 00" SEVERITY failure;
    sel <= "01";
    WAIT FOR 10 ns;
    ASSERT(output = "1010101010101010") REPORT "test failed for sel 01" SEVERITY failure;
    sel <= "10";
    WAIT FOR 10 ns;
    ASSERT(output = "0101010101010101") REPORT "test failed for sel 10" SEVERITY failure;
    sel <= "11";
    WAIT FOR 10 ns;
    ASSERT(output = "1111111111111111") REPORT "test failed for sel 11" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;