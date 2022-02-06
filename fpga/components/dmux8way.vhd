LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux8way IS
  PORT (
    input : IN STD_ULOGIC;
    sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0));
END dmux8way;

ARCHITECTURE structural OF dmux8way IS
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT sel8way
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0)
    );
  END COMPONENT;
  SIGNAL sel_out : STD_ULOGIC_VECTOR(7 DOWNTO 0);
BEGIN
  and_a : and_gate PORT MAP(input_a => input, input_b => sel_out(0), output => output(0));
  and_b : and_gate PORT MAP(input_a => input, input_b => sel_out(1), output => output(1));
  and_c : and_gate PORT MAP(input_a => input, input_b => sel_out(2), output => output(2));
  and_d : and_gate PORT MAP(input_a => input, input_b => sel_out(3), output => output(3));
  and_e : and_gate PORT MAP(input_a => input, input_b => sel_out(4), output => output(4));
  and_f : and_gate PORT MAP(input_a => input, input_b => sel_out(5), output => output(5));
  and_g : and_gate PORT MAP(input_a => input, input_b => sel_out(6), output => output(6));
  and_h : and_gate PORT MAP(input_a => input, input_b => sel_out(7), output => output(7));
  sel_a : sel8way PORT MAP(input => sel, output => sel_out);
END structural;