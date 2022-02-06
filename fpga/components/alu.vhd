LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY alu IS
  PORT (
    input_x : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    input_y : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    zero_x : IN STD_ULOGIC;
    zero_y : IN STD_ULOGIC;
    not_x : IN STD_ULOGIC;
    not_y : IN STD_ULOGIC;
    use_add : IN STD_ULOGIC;
    should_not_output : IN STD_ULOGIC;
    output, not_output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output_is_zero : OUT STD_ULOGIC
  );
END alu;

ARCHITECTURE structural OF alu IS
  COMPONENT mux16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT not16
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT and16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT add16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT is_non_zero
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL const_zero,
  x_maybe_zeroed,
  y_maybe_zeroed,
  x_notted,
  y_notted,
  x_maybe_notted,
  y_maybe_notted,
  anded,
  added,
  anded_or_added,
  notted,
  final_output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL output_is_non_zero : STD_ULOGIC;
BEGIN
  const_zero <= "0000000000000000";
  zero_x_mux : mux16 PORT MAP(
    input_a => input_x,
    input_b => const_zero,
    sel => zero_x,
    output => x_maybe_zeroed);
  zero_y_mux : mux16 PORT MAP(
    input_a => input_y,
    input_b => const_zero,
    sel => zero_y,
    output => y_maybe_zeroed);
  not16_x : not16 PORT MAP(input => x_maybe_zeroed, output => x_notted);
  not16_y : not16 PORT MAP(input => y_maybe_zeroed, output => y_notted);
  not_x_mux : mux16 PORT MAP(
    input_a => x_maybe_zeroed,
    input_b => x_notted,
    sel => not_x,
    output => x_maybe_notted);
  not_y_mux : mux16 PORT MAP(
    input_a => y_maybe_zeroed,
    input_b => y_notted,
    sel => not_y,
    output => y_maybe_notted);
  ander : and16 PORT MAP(input_a => x_maybe_notted, input_b => y_maybe_notted, output => anded);
  adder : add16 PORT MAP(input_a => x_maybe_notted, input_b => y_maybe_notted, output => added);
  op_mux : mux16 PORT MAP(input_a => anded, input_b => added, sel => use_add, output => anded_or_added);
  post_op_not : not16 PORT MAP(input => anded_or_added, output => notted);
  not_mux : mux16 PORT MAP(input_a => anded_or_added, input_b => notted, sel => should_not_output, output => final_output);
  output_not16 : not16 PORT MAP(input => final_output, output => not_output);
  is_non_zero_instance : is_non_zero PORT MAP(input => final_output, output => output_is_non_zero);
  is_zero_not : not_gate PORT MAP(input => output_is_non_zero, output => output_is_zero);
  output <= final_output;
END structural;