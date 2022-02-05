LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY not16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
END not16;

ARCHITECTURE structural OF not16 IS
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
BEGIN
  gen_not :
  FOR I IN 0 TO 15 GENERATE
    not_i : not_gate PORT MAP
      (input => input(i), output => output(i));
  END GENERATE gen_not;
END structural;