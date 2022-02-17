LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY ram4k IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(11 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END ram4k;

ARCHITECTURE structural OF ram4k IS
  COMPONENT ram512
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(8 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  COMPONENT dmux8way
    PORT (
      input : IN STD_ULOGIC;
      sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0));
  END COMPONENT;
  COMPONENT mux8way16
    PORT (
      input : IN STD_ULOGIC_VECTOR(127 DOWNTO 0);
      sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;
  SIGNAL mux_input : STD_ULOGIC_VECTOR(127 DOWNTO 0);
  SIGNAL dmux_output : STD_ULOGIC_VECTOR(7 DOWNTO 0);
BEGIN
  mux : mux8way16 PORT MAP(input => mux_input, sel => address(11 DOWNTO 9), output => output);
  dmux : dmux8way PORT MAP(input => load, sel => address(11 DOWNTO 9), output => dmux_output);
  gen_ram512 :
  FOR I IN 0 TO 7 GENERATE
    ram512_i : ram512 PORT MAP
      (input => input, address => address(8 DOWNTO 0), output => mux_input((i * 16) + 15 DOWNTO (i * 16)), load => dmux_output(i), clock => clock);
  END GENERATE gen_ram512;
END structural;