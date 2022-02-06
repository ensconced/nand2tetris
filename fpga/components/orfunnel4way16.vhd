LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY orfunnel4way16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(63 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END orfunnel4way16;

ARCHITECTURE structural OF orfunnel4way16 IS
  COMPONENT or16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;
  SIGNAL or_b_out, or_c_out : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  -- top layer
  or_a : or16 PORT MAP(input_a => or_b_out, input_b => or_c_out, output => output);
  -- bottom layer
  or_b : or16 PORT MAP(input_a => input(63 DOWNTO 48), input_b => input(47 DOWNTO 32), output => or_b_out);
  or_c : or16 PORT MAP(input_a => input(31 DOWNTO 16), input_b => input(15 DOWNTO 0), output => or_c_out);
END structural;