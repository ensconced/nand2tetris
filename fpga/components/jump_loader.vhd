LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY jump_loader IS
  PORT (
    j1 : IN STD_ULOGIC;
    j2 : IN STD_ULOGIC;
    j3 : IN STD_ULOGIC;
    is_zero : IN STD_ULOGIC;
    is_negative : IN STD_ULOGIC;
    is_c_instruction : IN STD_ULOGIC;
    output : OUT STD_ULOGIC);
END jump_loader;

ARCHITECTURE structural OF jump_loader IS
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT or3way
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC
    );
  END COMPONENT;
  SIGNAL is_not_zero, is_not_negative, is_positive, jump1, jump2, jump3, some_jump : STD_ULOGIC;
BEGIN
  not_a : not_gate PORT MAP
    (input => is_zero, output => is_not_zero);
  not_b : not_gate PORT MAP
    (input => is_negative, output => is_not_negative);
  and_a : and_gate PORT MAP
    (input_a => is_not_zero, input_b => is_not_negative, output => is_positive);
  and_b : and_gate PORT MAP
    (input_a => is_negative, input_b => j1, output => jump1);
  and_c : and_gate PORT MAP
    (input_a => is_zero, input_b => j2, output => jump2);
  and_d : and_gate PORT MAP
    (input_a => is_positive, input_b => j3, output => jump3);
  or3way_a : or3way PORT MAP(input(0) => jump1, input(1) => jump2, input(2) => jump3, output => some_jump);
  and_e : and_gate PORT MAP
    (input_a => some_jump, input_b => is_c_instruction, output => output);
END structural;