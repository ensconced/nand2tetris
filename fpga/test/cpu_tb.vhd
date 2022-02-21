LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE std.textio.ALL;

ENTITY cpu_tb IS
END cpu_tb;

ARCHITECTURE Behavioral OF cpu_tb IS
  PROCEDURE tick(
    SIGNAL clock : OUT STD_ULOGIC) IS BEGIN
    WAIT FOR 5 ns;
    clock <= '0';
    WAIT FOR 5 ns;
    clock <= '1';
    WAIT FOR 5 ns;
  END tick;
  COMPONENT cpu IS
    PORT (
      inM : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      instruction : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      reset : IN STD_ULOGIC;
      clock : IN STD_ULOGIC;
      outM : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      addressM : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      writeM : OUT STD_ULOGIC;
      pc : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;

  SIGNAL reset, writeM, clk : STD_ULOGIC;
  SIGNAL inM, instruction, outM, addressM, pc : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  uut : cpu PORT MAP(
    inM => inM,
    instruction => instruction,
    reset => reset,
    outM => outM,
    addressM => addressM,
    writeM => writeM,
    pc => pc,
    clock => clk
  );

  stim : PROCESS
  BEGIN
    -----------------------------------------------------------------------------------
    -- SETUP
    inM <= "0000000000000000";
    instruction <= "0000000000000000";
    reset <= '1';
    tick(clk);
    ASSERT (pc = "0000000000000000") REPORT "failed test at stage 1" SEVERITY failure;
    -----------------------------------------------------------------------------------
    -- INCREMENTING PC
    reset <= '0';
    tick(clk);
    ASSERT (pc = "0000000000000001") REPORT "failed test at stage 2" SEVERITY failure;
    tick(clk);
    ASSERT (pc = "0000000000000010") REPORT "failed test at stage 3" SEVERITY failure;
    -----------------------------------------------------------------------------------
    -- A + D
    -- load value "1" into register A
    instruction <= "0000000000000001";
    tick(clk);
    ASSERT (pc = "0000000000000011") REPORT "failed test at stage 4" SEVERITY failure;
    ASSERT (addressM = "0000000000000001") REPORT "failed test at stage 5" SEVERITY failure;
    -- load contents of register A into register D
    instruction <= "1110110000010000";
    tick(clk);
    ASSERT (outM = "0000000000000001") REPORT "failed test at stage 6" SEVERITY failure;
    -- load value "2" into register A
    instruction <= "0000000000000010";
    tick(clk);
    ASSERT (addressM = "0000000000000010") REPORT "failed test at stage 7" SEVERITY failure;
    -- add the values of registers A and D, and on next tick load result into register A
    instruction <= "1110000010100000";
    WAIT FOR 5 ns;
    ASSERT (outM = "0000000000000011") REPORT "failed test at stage 8" SEVERITY failure;
    tick(clk);
    ASSERT (addressM = "0000000000000011") REPORT "failed test at stage 9" SEVERITY failure;
    -----------------------------------------------------------------------------------
    -- D + M
    -- load value "3" into register A
    instruction <= "0000000000000011";
    tick(clk);
    ASSERT (addressM = "0000000000000011") REPORT "failed test at stage 10" SEVERITY failure;
    -- load contents of register A into register D
    instruction <= "1110110000010000";
    tick(clk);
    ASSERT (outM = "0000000000000011") REPORT "failed test at stage 11" SEVERITY failure;
    -- set value "4" as inM
    inM <= "0000000000000100";
    -- add the values of D and inM, and load result into memory
    instruction <= "1111000010001000";
    WAIT FOR 5 ns;
    ASSERT (outM = "0000000000000111") REPORT "failed test at stage 12" SEVERITY failure;
    ASSERT (writeM = '1') REPORT "failed test at stage 13" SEVERITY failure;
    -----------------------------------------------------------------------------------
    -- 10 x 10
    -- load value "10" into register A
    instruction <= "0000000000001010";
    tick(clk);
    -- load contents of register A into register D
    instruction <= "1110110000010000";
    tick(clk);
    ASSERT (outM = "0000000000001010") REPORT "failed test at stage 13" SEVERITY failure;
    -- repeat 9 x (compute A + D and load result into register D)
    instruction <= "1110000010010000";
    WAIT FOR 5 ns;
    ASSERT (outM = "0000000000010100") REPORT "failed test at stage 14" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000000011110") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000000101000") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000000110010") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000000111100") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000001000110") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000001010000") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000001011010") REPORT "failed test at stage 15" SEVERITY failure;
    tick(clk);
    ASSERT (outM = "0000000001100100") REPORT "failed test at stage 15" SEVERITY failure;
    -- TODO - j1j2j3
    WAIT;
  END PROCESS;
END Behavioral;