LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY counter_tb IS
END counter_tb;

ARCHITECTURE Behavioral OF counter_tb IS
  PROCEDURE tick(
    SIGNAL clock : OUT STD_ULOGIC) IS BEGIN
    WAIT FOR 5 ns;
    clock <= '0';
    WAIT FOR 5 ns;
    clock <= '1';
    WAIT FOR 5 ns;
  END tick;
  COMPONENT counter IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      inc : IN STD_ULOGIC;
      load : IN STD_ULOGIC;
      reset : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL inc, load, reset, clock : STD_ULOGIC;
BEGIN
  uut : counter PORT MAP(
    input => input,
    output => output,
    inc => inc,
    load => load,
    reset => reset,
    clock => clock
  );
  stim : PROCESS
  BEGIN
    inc <= '0';
    load <= '1';
    reset <= '0';
    input <= "0000000000000000";
    tick(clock);
    ASSERT (output = "0000000000000000") REPORT "test failed at stage 1" SEVERITY failure;
    -- test incrementing
    load <= '0';
    inc <= '1';
    tick(clock);
    ASSERT (output = "0000000000000001") REPORT "test failed at stage 2" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000010") REPORT "test failed at stage 3" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000011") REPORT "test failed at stage 4" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000100") REPORT "test failed at stage 5" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000101") REPORT "test failed at stage 6" SEVERITY failure;
    -- test loading
    inc <= '0';
    load <= '1';
    input <= "1000000000000000";
    tick(clock);
    ASSERT (output = "1000000000000000") REPORT "test failed at stage 7" SEVERITY failure;
    tick(clock);
    ASSERT (output = "1000000000000000") REPORT "test failed at stage 8" SEVERITY failure;
    load <= '0';
    inc <= '1';
    tick(clock);
    ASSERT (output = "1000000000000001") REPORT "test failed at stage 9" SEVERITY failure;
    tick(clock);
    ASSERT (output = "1000000000000010") REPORT "test failed at stage 10" SEVERITY failure;
    -- test resetting
    load <= '0';
    inc <= '0';
    reset <= '1';
    tick(clock);
    ASSERT (output = "0000000000000000") REPORT "test failed at stage 11" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000000") REPORT "test failed at stage 12" SEVERITY failure;
    reset <= '0';
    inc <= '1';
    tick(clock);
    ASSERT (output = "0000000000000001") REPORT "test failed at stage 13" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000010") REPORT "test failed at stage 14" SEVERITY failure;
    tick(clock);
    ASSERT (output = "0000000000000011") REPORT "test failed at stage 15" SEVERITY failure;
  END PROCESS;
END Behavioral;