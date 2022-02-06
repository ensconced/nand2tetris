LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY sel8way IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0)
  );
END sel8way;

ARCHITECTURE structural OF sel8way IS
  COMPONENT and3way
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL not_a_out, not_b_out, not_c_out : STD_ULOGIC;
BEGIN
  not_a : not_gate PORT MAP(input => input(0), output => not_a_out);
  not_b : not_gate PORT MAP(input => input(1), output => not_b_out);
  not_c : not_gate PORT MAP(input => input(2), output => not_c_out);
  and_a : and3way PORT MAP(input => (not_c_out & not_b_out & not_a_out), output => output(0));
  and_b : and3way PORT MAP(input => (not_c_out & not_b_out & input(0)), output => output(1));
  and_c : and3way PORT MAP(input => (not_c_out & input(1) & not_a_out), output => output(2));
  and_d : and3way PORT MAP(input => (not_c_out & input(1) & input(0)), output => output(3));
  and_e : and3way PORT MAP(input => (input(2) & not_b_out & not_a_out), output => output(4));
  and_f : and3way PORT MAP(input => (input(2) & not_b_out & input(0)), output => output(5));
  and_g : and3way PORT MAP(input => (input(2) & input(1) & not_a_out), output => output(6));
  and_h : and3way PORT MAP(input => (input(2) & input(1) & input(0)), output => output(7));
END structural;