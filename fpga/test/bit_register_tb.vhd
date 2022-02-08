LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY bit_register_tb IS
END bit_register_tb;

ARCHITECTURE Behavioral OF bit_register_tb IS
  COMPONENT bit_register IS
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC;
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output, load, clock : STD_ULOGIC;
BEGIN
  uut : bit_register PORT MAP(
    input => input,
    output => output,
    load => load,
    clock => clock
  );
  stim : PROCESS
  BEGIN
    -- 0
    clock <= '0';
    load <= '1';
    input <= '1';
    WAIT FOR 5 ns;
    -- 1
    WAIT FOR 5 ns;
    -- 2
    clock <= '1';
    WAIT FOR 5 ns;
    -- 3
    load <= '0';
    input <= '0';
    ASSERT (output = '1') REPORT "test failed at stage 3" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 4
    clock <= '0';
    ASSERT (output = '1') REPORT "test failed at stage 4" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 5
    ASSERT (output = '1') REPORT "test failed at stage 5" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 6
    clock <= '1';
    ASSERT (output = '1') REPORT "test failed at stage 6" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 7
    ASSERT (output = '1') REPORT "test failed at stage 7" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 8
    clock <= '0';
    ASSERT (output = '1') REPORT "test failed at stage 8" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 9
    load <= '1';
    ASSERT (output = '1') REPORT "test failed at stage 9" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 10
    clock <= '1';
    WAIT FOR 5 ns;
    -- 11
    ASSERT (output = '0') REPORT "test failed at stage 11" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 12
    clock <= '0';
    ASSERT (output = '0') REPORT "test failed at stage 12" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 13
    load <= '0';
    input <= '1';
    ASSERT (output = '0') REPORT "test failed at stage 13" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 14
    clock <= '1';
    ASSERT (output = '0') REPORT "test failed at stage 14" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 15
    ASSERT (output = '0') REPORT "test failed at stage 15" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 16
    clock <= '0';
    ASSERT (output = '0') REPORT "test failed at stage 16" SEVERITY failure;
    WAIT FOR 5 ns;
    -- 17
    ASSERT (output = '0') REPORT "test failed at stage 17" SEVERITY failure;
    WAIT FOR 5 ns;
  END PROCESS;
END Behavioral;