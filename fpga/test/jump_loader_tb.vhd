LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY jump_loader_tb IS
END jump_loader_tb;

ARCHITECTURE Behavioral OF jump_loader_tb IS
  COMPONENT jump_loader IS
    PORT (
      j1 : IN STD_ULOGIC;
      j2 : IN STD_ULOGIC;
      j3 : IN STD_ULOGIC;
      is_zero : IN STD_ULOGIC;
      is_negative : IN STD_ULOGIC;
      is_c_instruction : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL j1, j2, j3, is_zero, is_negative, is_c_instruction, output : STD_ULOGIC;
BEGIN
  uut : jump_loader PORT MAP(
    j1 => j1,
    j2 => j2,
    j3 => j3,
    is_zero => is_zero,
    is_negative => is_negative,
    is_c_instruction => is_c_instruction,
    output => output
  );
  stim : PROCESS
  BEGIN
    -- TODO - also test when this is set to zero
    is_c_instruction <= '1';
    -----------------------------------------------------------------------
    -- Jump iff negative
    j1 <= '1';
    j2 <= '0';
    j3 <= '0';
    -- jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '1') REPORT "test failed at stage 1" SEVERITY failure;

    -- no jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 2" SEVERITY failure;

    -- no jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 3" SEVERITY failure;
    -----------------------------------------------------------------------
    -- Jump iff zero
    j1 <= '0';
    j2 <= '1';
    j3 <= '0';

    -- no jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 4" SEVERITY failure;

    -- jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '1') REPORT "test failed at stage 5" SEVERITY failure;

    -- no jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 6" SEVERITY failure;

    -----------------------------------------------------------------------
    -- Jump iff positive
    j1 <= '0';
    j2 <= '0';
    j3 <= '1';

    -- no jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 7" SEVERITY failure;

    -- no jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 8" SEVERITY failure;

    -- jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '1') REPORT "test failed at stage 9" SEVERITY failure;

    -----------------------------------------------------------------------
    -- When is_c_instruction is zero, we should get no jumps under any circumstance.
    is_c_instruction <= '0';
    -----------------------------------------------------------------------
    j1 <= '1';
    j2 <= '0';
    j3 <= '0';
    -- no jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 1" SEVERITY failure;

    -- no jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 2" SEVERITY failure;

    -- no jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 3" SEVERITY failure;
    -----------------------------------------------------------------------
    j1 <= '0';
    j2 <= '1';
    j3 <= '0';

    -- no jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 4" SEVERITY failure;

    -- no jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 5" SEVERITY failure;

    -- no jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 6" SEVERITY failure;

    -----------------------------------------------------------------------
    j1 <= '0';
    j2 <= '0';
    j3 <= '1';

    -- no jump for negative
    is_negative <= '1';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 7" SEVERITY failure;

    -- no jump for zero
    is_negative <= '0';
    is_zero <= '1';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 8" SEVERITY failure;

    -- no jump for positive
    is_negative <= '0';
    is_zero <= '0';
    WAIT FOR 5 ns;
    ASSERT (output = '0') REPORT "test failed at stage 9" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;