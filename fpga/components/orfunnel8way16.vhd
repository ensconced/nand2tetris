LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY orfunnel8way16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(127 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END orfunnel8way16;

ARCHITECTURE structural OF orfunnel8way16 IS
  COMPONENT or16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;
  SIGNAL or_b_out, or_c_out, or_d_out, or_e_out, or_f_out, or_g_out : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  -- top layer
  or_a : or16 PORT MAP(input_a => or_b_out, input_b => or_c_out, output => output);
  -- middle layer
  or_b : or16 PORT MAP(input_a => or_d_out, input_b => or_e_out, output => or_b_out);
  or_c : or16 PORT MAP(input_a => or_f_out, input_b => or_g_out, output => or_c_out);
  -- bottom layer
  or_d : or16 PORT MAP(input_a => input(127 DOWNTO 112), input_b => input(111 DOWNTO 96), output => or_d_out);
  or_e : or16 PORT MAP(input_a => input(95 DOWNTO 80), input_b => input(79 DOWNTO 64), output => or_e_out);
  or_f : or16 PORT MAP(input_a => input(63 DOWNTO 48), input_b => input(47 DOWNTO 32), output => or_f_out);
  or_g : or16 PORT MAP(input_a => input(31 DOWNTO 16), input_b => input(15 DOWNTO 0), output => or_g_out);
END structural;