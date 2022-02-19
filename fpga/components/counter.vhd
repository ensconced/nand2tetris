LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY counter IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    inc : IN STD_ULOGIC;
    load : IN STD_ULOGIC;
    reset : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END counter;

ARCHITECTURE structural OF counter IS
  COMPONENT mux16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT register16
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  COMPONENT inc16
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT or3way
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL or_out : STD_ULOGIC;
  SIGNAL mux_a_out, mux_b_out, mux_c_out, reg_out, inc_out, const_zero : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  const_zero <= "0000000000000000";
  mux_a : mux16 PORT MAP
    (input_a => mux_b_out, input_b => const_zero, sel => reset, output => mux_a_out);
  mux_b : mux16 PORT MAP
    (input_a => mux_c_out, input_b => input, sel => load, output => mux_b_out);
  mux_c : mux16 PORT MAP
    (input_a => reg_out, input_b => inc_out, sel => inc, output => mux_c_out);
  reg16 : register16 PORT MAP(input => mux_a_out, output => reg_out, load => or_out, clock => clock);
  or3way_a : or3way PORT MAP(input(0) => inc, input(1) => load, input(2) => reset, output => or_out);
  inc_a : inc16 PORT MAP(input => reg_out, output => inc_out);
  output <= reg_out;
END structural;