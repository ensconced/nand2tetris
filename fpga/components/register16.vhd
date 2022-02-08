LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY register16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END register16;

ARCHITECTURE structural OF register16 IS
  COMPONENT bit_register
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC;
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
BEGIN
  gen_reg :
  FOR I IN 0 TO 15 GENERATE
    reg_i : bit_register PORT MAP
      (input => input(i), output => output(i), load => load, clock => clock);
  END GENERATE gen_reg;
END structural;