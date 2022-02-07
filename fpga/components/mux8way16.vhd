LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux8way16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(127 DOWNTO 0);
    sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END mux8way16;

ARCHITECTURE structural OF mux8way16 IS
  COMPONENT sel8way
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(7 DOWNTO 0)
    );
  END COMPONENT;
  COMPONENT orfunnel8way16
    PORT (
      input : IN STD_ULOGIC_VECTOR(127 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;
  COMPONENT mux16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  SIGNAL const_zero, mux_a_out, mux_b_out, mux_c_out, mux_d_out, mux_e_out, mux_f_out, mux_g_out, mux_h_out : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL sel_out : STD_ULOGIC_VECTOR(7 DOWNTO 0);
BEGIN
  const_zero <= "0000000000000000";
  sel_a : sel8way PORT MAP(input => sel, output => sel_out);
  mux_a : mux16 PORT MAP(input_a => const_zero, input_b => input(15 DOWNTO 0), sel => sel_out(0), output => mux_a_out);
  mux_b : mux16 PORT MAP(input_a => const_zero, input_b => input(31 DOWNTO 16), sel => sel_out(1), output => mux_b_out);
  mux_c : mux16 PORT MAP(input_a => const_zero, input_b => input(47 DOWNTO 32), sel => sel_out(2), output => mux_c_out);
  mux_d : mux16 PORT MAP(input_a => const_zero, input_b => input(63 DOWNTO 48), sel => sel_out(3), output => mux_d_out);
  mux_e : mux16 PORT MAP(input_a => const_zero, input_b => input(79 DOWNTO 64), sel => sel_out(4), output => mux_e_out);
  mux_f : mux16 PORT MAP(input_a => const_zero, input_b => input(95 DOWNTO 80), sel => sel_out(5), output => mux_f_out);
  mux_g : mux16 PORT MAP(input_a => const_zero, input_b => input(111 DOWNTO 96), sel => sel_out(6), output => mux_g_out);
  mux_h : mux16 PORT MAP(input_a => const_zero, input_b => input(127 DOWNTO 112), sel => sel_out(7), output => mux_h_out);
  or_funnel : orfunnel8way16 PORT MAP(
    input(127 DOWNTO 112) => mux_h_out,
    input(111 DOWNTO 96) => mux_g_out,
    input(95 DOWNTO 80) => mux_f_out,
    input(79 DOWNTO 64) => mux_e_out,
    input(63 DOWNTO 48) => mux_d_out,
    input(47 DOWNTO 32) => mux_c_out,
    input(31 DOWNTO 16) => mux_b_out,
    input(15 DOWNTO 0) => mux_a_out,
    output => output);
END structural;