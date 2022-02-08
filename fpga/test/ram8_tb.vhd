LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY ram8_tb IS
END ram8_tb;

ARCHITECTURE Behavioral OF ram8_tb IS
  PROCEDURE load_value(
    VARIABLE value : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    SIGNAL clock : OUT STD_ULOGIC;
    SIGNAL input : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    SIGNAL load : OUT STD_ULOGIC) IS BEGIN
    clock <= '0';
    WAIT FOR 5 ns;
    input <= value;
    load <= '1';
    WAIT FOR 5 ns;
    clock <= '1';
    WAIT FOR 5 ns;
  END load_value;
  COMPONENT ram8 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL address : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL load, clock : STD_ULOGIC;
BEGIN
  uut : ram8 PORT MAP(
    input => input,
    output => output,
    address => address,
    load => load,
    clock => clock
  );
  stim : PROCESS
    VARIABLE a : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_signed(-32768, 16));
    VARIABLE b : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_signed(-5463, 16));
    VARIABLE c : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_signed(-32767, 16));
  BEGIN
    address <= "000";
    load_value(value => a, clock => clock, input => input, load => load);
    ASSERT (output = a) REPORT "test failed at stage 1" SEVERITY failure;
  END PROCESS;
END Behavioral;