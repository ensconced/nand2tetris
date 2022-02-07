LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY flip_flop_tb IS
END flip_flop_tb;

ARCHITECTURE Behavioral OF flip_flop_tb IS
  COMPONENT flip_flop IS
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output, clock : STD_ULOGIC;

BEGIN
  uut : flip_flop PORT MAP(
    input => input,
    output => output,
    clock => clock
  );
  stim : PROCESS
  BEGIN
    -- 0
    clock <= '0';
    input <= '0';
    WAIT FOR 5 ns;
    -- 1
    input <= '1';
    WAIT FOR 5 ns;
    -- 2
    clock <= '1';
    WAIT FOR 5 ns;
    -- 3
    input <= '0';
    ASSERT (output = '1') REPORT "failed test at stage 3" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 4
    clock <= '0';
    ASSERT (output = '1') REPORT "failed test at stage 4" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 5
    ASSERT (output = '1') REPORT "failed test at stage 5" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 6
    clock <= '1';
    WAIT FOR 5 ns;
    -- 7
    input <= '1';
    ASSERT (output = '0') REPORT "failed test at stage 7" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 8
    clock <= '0';
    ASSERT (output = '0') REPORT "failed test at stage 8" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 9
    input <= '0';
    ASSERT (output = '0') REPORT "failed test at stage 9" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 10
    clock <= '1';
    ASSERT (output = '0') REPORT "failed test at stage 10" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 11
    ASSERT (output = '0') REPORT "failed test at stage 11" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 12
    clock <= '0';
    ASSERT (output = '0') REPORT "failed test at stage 12" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 13
    input <= '1';
    ASSERT (output = '0') REPORT "failed test at stage 13" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 14
    clock <= '1';
    ASSERT (output = '0') REPORT "failed test at stage 14" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 15
    ASSERT (output = '1') REPORT "failed test at stage 15" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 16
    clock <= '0';
    ASSERT (output = '1') REPORT "failed test at stage 16" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 17
    input <= '0';
    ASSERT (output = '1') REPORT "failed test at stage 17" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 18
    clock <= '1';
    WAIT FOR 5 ns;
    -- 19
    ASSERT (output = '0') REPORT "failed test at stage 19" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 20
    clock <= '0';
    ASSERT (output = '0') REPORT "failed test at stage 20" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 21
    ASSERT (output = '0') REPORT "failed test at stage 21" SEVERITY failure;
    WAIT FOR 5 ns;
  END PROCESS;
END Behavioral;