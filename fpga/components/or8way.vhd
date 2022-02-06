LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY or8way IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(7 DOWNTO 0);
    output : OUT STD_ULOGIC
  );
END or8way;

ARCHITECTURE structural OF or8way IS
  COMPONENT or_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL or_b_out, or_c_out, or_d_out, or_e_out, or_f_out, or_g_out : STD_ULOGIC;
BEGIN
  -- top layer
  or_a : or_gate PORT MAP(input_a => or_b_out, input_b => or_c_out, output => output);
  -- middle layer
  or_b : or_gate PORT MAP(input_a => or_d_out, input_b => or_e_out, output => or_b_out);
  or_c : or_gate PORT MAP(input_a => or_f_out, input_b => or_g_out, output => or_c_out);
  -- bottom layer
  or_d : or_gate PORT MAP(input_a => input(0), input_b => input(1), output => or_d_out);
  or_e : or_gate PORT MAP(input_a => input(2), input_b => input(3), output => or_e_out);
  or_f : or_gate PORT MAP(input_a => input(4), input_b => input(5), output => or_f_out);
  or_g : or_gate PORT MAP(input_a => input(6), input_b => input(7), output => or_g_out);
END structural;