LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY ram8 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END ram8;

ARCHITECTURE structural OF ram8 IS
  COMPONENT register16
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
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
  mux : mux8way16 PORT MAP(input => mux_input, sel => address, output => output);
  dmux : dmux8way PORT MAP(input => load, sel => address, output => dmux_output);
  gen_reg :
  FOR I IN 0 TO 7 GENERATE
    reg_i : register16 PORT MAP
      (input => input, output => mux_input((i * 16) + 15 DOWNTO (i * 16)), load => dmux_output(i), clock => clock);
  END GENERATE gen_reg;
END structural;