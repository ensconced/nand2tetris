LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY ram512_tb IS
END ram512_tb;

ARCHITECTURE Behavioral OF ram512_tb IS
  PROCEDURE load_value(
    VARIABLE value : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    CONSTANT address_value : IN STD_ULOGIC_VECTOR(8 DOWNTO 0);
    SIGNAL address : OUT STD_ULOGIC_VECTOR(8 DOWNTO 0);
    SIGNAL clock : OUT STD_ULOGIC;
    SIGNAL input : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    SIGNAL load : OUT STD_ULOGIC) IS BEGIN
    address <= address_value;
    clock <= '0';
    WAIT FOR 5 ns;
    input <= value;
    load <= '1';
    WAIT FOR 5 ns;
    clock <= '1';
    WAIT FOR 5 ns;
  END load_value;
  PROCEDURE check_value(
    VARIABLE expected_value : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    CONSTANT address_value : IN STD_ULOGIC_VECTOR(8 DOWNTO 0);
    SIGNAL address : OUT STD_ULOGIC_VECTOR(8 DOWNTO 0);
    SIGNAL output : IN STD_ULOGIC_VECTOR(15 DOWNTO 0)) IS BEGIN
    address <= address_value;
    WAIT FOR 5 ns;
    ASSERT (output = expected_value) REPORT "test failed" SEVERITY failure;
  END check_value;
  COMPONENT ram512 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(8 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  SIGNAL input, output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL address : STD_ULOGIC_VECTOR(8 DOWNTO 0);
  SIGNAL load, clock : STD_ULOGIC;
BEGIN
  uut : ram512 PORT MAP(
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
    VARIABLE d : STD_ULOGIC_VECTOR(15 DOWNTO 0) := STD_ULOGIC_VECTOR(to_signed(1234, 16));
  BEGIN
    load_value(a, "101010101", address, clock, input, load);
    check_value(a, "101010101", address, output);
    load_value(b, "000111000", address, clock, input, load);
    check_value(a, "101010101", address, output);
    check_value(b, "000111000", address, output);
    load_value(c, "111000111", address, clock, input, load);
    check_value(a, "101010101", address, output);
    check_value(b, "000111000", address, output);
    check_value(c, "111000111", address, output);
    load_value(d, "000000100", address, clock, input, load);
    check_value(a, "101010101", address, output);
    check_value(b, "000111000", address, output);
    check_value(c, "111000111", address, output);
    check_value(d, "000000100", address, output);
  END PROCESS;
END Behavioral;