LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY ram16k_switch_test IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(6 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END ram16k_switch_test;

ARCHITECTURE structural OF ram16k_switch_test IS
  COMPONENT ram16k
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(13 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  SIGNAL const_zero : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  const_zero <= "0000000000000000";
  ram16k_a : ram16k PORT MAP(
    input(15 DOWNTO 8) => const_zero(15 DOWNTO 8),
    input(7 DOWNTO 0) => input,
    address(13 DOWNTO 7) => address,
    address(6 DOWNTO 0) => const_zero(6 DOWNTO 0),
    output => output,
    load => load,
    clock => clock
  );
END structural;