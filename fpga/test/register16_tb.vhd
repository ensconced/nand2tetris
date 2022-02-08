LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY register16_tb IS
END register16_tb;

ARCHITECTURE Behavioral OF register16_tb IS
  COMPONENT register16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL load, clock : STD_ULOGIC;
BEGIN
  uut : register16 PORT MAP(
    input => input,
    output => output,
    load => load,
    clock => clock
  );
  stim : PROCESS
    TYPE test_inputs IS ARRAY(5 DOWNTO 0) OF signed(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (
      to_signed(-32768, 16),
      to_signed(-681, 16),
      to_signed(0, 16),
      to_signed(1, 16),
      to_signed(5463, 16),
      to_signed(32767, 16)
    );
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      -- 0
      clock <= '0';
      load <= '1';
      input <= STD_ULOGIC_VECTOR(test_nums(i));
      WAIT FOR 5 ns;
      -- 1
      WAIT FOR 5 ns;
      -- 2
      clock <= '1';
      WAIT FOR 5 ns;
      -- 3
      load <= '0';
      input <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 3" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 4
      clock <= '0';
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 4" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 5
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 5" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 6
      clock <= '1';
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 6" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 7
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 7" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 8
      clock <= '0';
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 8" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 9
      load <= '1';
      ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "test failed at stage 9" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 10
      clock <= '1';
      WAIT FOR 5 ns;
      -- 11
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 11" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 12
      clock <= '0';
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 12" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 13
      load <= '0';
      input <= STD_ULOGIC_VECTOR(test_nums(i));
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 13" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 14
      clock <= '1';
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 14" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 15
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 15" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 16
      clock <= '0';
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 16" SEVERITY failure;
      WAIT FOR 5 ns;
      -- 17
      ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed at stage 17" SEVERITY failure;
      WAIT FOR 5 ns;
    END LOOP;
  END PROCESS;
END Behavioral;