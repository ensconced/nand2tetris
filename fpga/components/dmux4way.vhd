LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY dmux4way IS
  PORT (
    input : IN STD_ULOGIC;
    sel : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0));
END dmux4way;

ARCHITECTURE structural OF dmux4way IS
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT sel4way
    PORT (
      input : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0)
    );
  END COMPONENT;
  SIGNAL sel_out : STD_ULOGIC_VECTOR(3 DOWNTO 0);
BEGIN
  and_a : and_gate PORT MAP(input_a => input, input_b => sel_out(0), output => output(0));
  and_b : and_gate PORT MAP(input_a => input, input_b => sel_out(1), output => output(1));
  and_c : and_gate PORT MAP(input_a => input, input_b => sel_out(2), output => output(2));
  and_d : and_gate PORT MAP(input_a => input, input_b => sel_out(3), output => output(3));
  sel_a : sel4way PORT MAP(input => sel, output => sel_out);
END structural;