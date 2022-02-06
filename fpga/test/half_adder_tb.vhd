LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY half_adder_tb IS
END half_adder_tb;

ARCHITECTURE Behavioral OF half_adder_tb IS
  COMPONENT half_adder IS
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
  END COMPONENT;

  SIGNAL a, b : STD_ULOGIC;
  SIGNAL output : STD_ULOGIC_VECTOR(1 DOWNTO 0);

BEGIN
  uut : half_adder PORT MAP(
    input_a => a,
    input_b => b,
    output => output
  );

  stim : PROCESS
  BEGIN
    a <= '0';
    b <= '0';
    WAIT FOR 10 ns;
    ASSERT (output = "00") REPORT "failed for input_a: 0, input_b: 0" SEVERITY failure;
    a <= '0';
    b <= '1';
    WAIT FOR 10 ns;
    ASSERT (output = "01") REPORT "failed for input_a: 0, input_b: 1" SEVERITY failure;
    a <= '1';
    b <= '0';
    WAIT FOR 10 ns;
    ASSERT (output = "01") REPORT "failed for input_a: 1, input_b: 0" SEVERITY failure;
    a <= '1';
    b <= '1';
    WAIT FOR 10 ns;
    ASSERT (output = "10") REPORT "failed for input_a: 1, input_b: 0" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;