LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux4way16 IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(63 DOWNTO 0);
    sel : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END mux4way16;

ARCHITECTURE structural OF mux4way16 IS
  COMPONENT sel4way
    PORT (
      input : IN STD_ULOGIC_VECTOR(1 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(3 DOWNTO 0)
    );
  END COMPONENT;
  COMPONENT orfunnel4way16
    PORT (
      input : IN STD_ULOGIC_VECTOR(63 DOWNTO 0);
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
  SIGNAL const_zero, mux_a_out, mux_b_out, mux_c_out, mux_d_out : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL sel_out : STD_ULOGIC_VECTOR(3 DOWNTO 0);
BEGIN
  const_zero <= "0000000000000000";
  sel_a : sel4way PORT MAP(input => sel, output => sel_out);
  mux_a : mux16 PORT MAP(input_a => const_zero, input_b => input(15 DOWNTO 0), sel => sel_out(0), output => mux_a_out);
  mux_b : mux16 PORT MAP(input_a => const_zero, input_b => input(31 DOWNTO 16), sel => sel_out(1), output => mux_b_out);
  mux_c : mux16 PORT MAP(input_a => const_zero, input_b => input(47 DOWNTO 32), sel => sel_out(2), output => mux_c_out);
  mux_d : mux16 PORT MAP(input_a => const_zero, input_b => input(63 DOWNTO 48), sel => sel_out(3), output => mux_d_out);
  or_funnel : orfunnel4way16 PORT MAP(
    input(63 DOWNTO 48) => mux_d_out,
    input(47 DOWNTO 32) => mux_c_out,
    input(31 DOWNTO 16) => mux_b_out,
    input(15 DOWNTO 0) => mux_a_out,
    output => output);
END structural;