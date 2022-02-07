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
    WAIT FOR 5 ns;
    -- 1
    clock <= '1';
    WAIT FOR 5 ns;
    -- 2
    clock <= '0';
    WAIT FOR 5 ns;
    -- 3
    clock <= '1';
    WAIT FOR 5 ns;
    -- 4
    clock <= '0';
    WAIT FOR 5 ns;
    -- 5
    clock <= '1';
    WAIT FOR 5 ns;
    -- 6
    clock <= '0';
    WAIT FOR 5 ns;
    -- 7
    clock <= '1';
    WAIT FOR 5 ns;
    -- 8
    clock <= '0';
    WAIT FOR 5 ns;
    -- 9
    clock <= '1';
    WAIT FOR 5 ns;
    -- 10
    clock <= '0';
    WAIT FOR 5 ns;
    -- 11
    clock <= '1';
    WAIT FOR 5 ns;
    -- 12
    clock <= '0';
    WAIT FOR 5 ns;
    -- 13
    clock <= '1';
    WAIT FOR 5 ns;
    -- 14
    clock <= '0';
    WAIT FOR 5 ns;
    -- 15
    clock <= '1';
    WAIT FOR 5 ns;
  END PROCESS;
END Behavioral;