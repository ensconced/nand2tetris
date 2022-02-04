ENTITY or_gate_tb IS
END or_gate_tb;

ARCHITECTURE Behavioral OF or_gate_tb IS
  COMPONENT or_gate IS
    PORT (
      input_a : IN BIT;
      input_b : IN BIT;
      output : OUT BIT);
  END COMPONENT;

  SIGNAL a, b, c : BIT;

BEGIN
  uut : or_gate PORT MAP(
    input_a => a,
    input_b => b,
    output => c
  );

  stim : PROCESS
  BEGIN
    a <= '0';
    b <= '0';
    WAIT FOR 10 ns;
    ASSERT (c = '0')
    REPORT "test failed for combination 00" SEVERITY failure;
    a <= '0';
    b <= '1';
    WAIT FOR 10 ns;
    ASSERT (c = '1')
    REPORT "test failed for combination 01" SEVERITY failure;

    a <= '1';
    b <= '1';
    WAIT FOR 10ns;
    ASSERT (c = '1')
    REPORT "test failed for combination 11" SEVERITY failure;

    a <= '1';
    b <= '0';
    WAIT FOR 10 ns;
    ASSERT (c = '1')
    REPORT "test failed for combination 10" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;