LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY orfunnel4way16_tb IS
END orfunnel4way16_tb;

ARCHITECTURE Behavioral OF orfunnel4way16_tb IS
  COMPONENT orfunnel4way16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(63 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;

  SIGNAL input : STD_ULOGIC_VECTOR(63 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(15 DOWNTO 0);

BEGIN
  uut : orfunnel4way16 PORT MAP(
    input => input,
    output => output
  );
  stim : PROCESS
  BEGIN
    input(63 DOWNTO 48) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(47 DOWNTO 32) <= STD_ULOGIC_VECTOR(to_unsigned(65535, 16));
    input(31 DOWNTO 16) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(15 DOWNTO 0) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    WAIT FOR 10 ns;
    ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(65535, 16))) REPORT "test failed" SEVERITY failure;
    WAIT;
    input(63 DOWNTO 48) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(47 DOWNTO 32) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(31 DOWNTO 16) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(15 DOWNTO 0) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    WAIT FOR 10 ns;
    ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "test failed" SEVERITY failure;
    input(63 DOWNTO 48) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(47 DOWNTO 32) <= STD_ULOGIC_VECTOR(to_unsigned(1234, 16));
    input(31 DOWNTO 16) <= STD_ULOGIC_VECTOR(to_unsigned(0, 16));
    input(15 DOWNTO 0) <= STD_ULOGIC_VECTOR(to_unsigned(123, 16));
    WAIT FOR 10 ns;
    ASSERT (output = (STD_ULOGIC_VECTOR(to_unsigned(1234, 16)) OR STD_ULOGIC_VECTOR(to_unsigned(123, 16)))) REPORT "test failed" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;