LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
ENTITY memory IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(14 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END memory;

ARCHITECTURE structural OF memory IS
  COMPONENT ram16k
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(13 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;

  -- TODO

  SIGNAL mux_input : STD_ULOGIC_VECTOR(63 DOWNTO 0);
  SIGNAL dmux_output : STD_ULOGIC_VECTOR(3 DOWNTO 0);
BEGIN
  mux : mux4way16 PORT MAP(input => mux_input, sel => address(13 DOWNTO 12), output => output);
  dmux : dmux4way PORT MAP(input => load, sel => address(13 DOWNTO 12), output => dmux_output);
  gen_ram4k :
  FOR I IN 0 TO 3 GENERATE
    ram4k_i : ram4k PORT MAP
      (input => input, address => address(11 DOWNTO 0), output => mux_input((i * 16) + 15 DOWNTO (i * 16)), load => dmux_output(i), clock => clock);
  END GENERATE gen_ram4k;
END structural;