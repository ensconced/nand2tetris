LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY bit_register IS
  PORT (
    input : IN STD_ULOGIC;
    output : OUT STD_ULOGIC;
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END bit_register;

ARCHITECTURE structural OF bit_register IS
  COMPONENT mux
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT flip_flop
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  SIGNAL ff_out, mux_out : STD_ULOGIC;
BEGIN
  mux_a : mux PORT MAP(input_a => ff_out, input_b => input, sel => load, output => mux_out);
  ff : flip_flop PORT MAP(input => mux_out, output => ff_out, clock => clock);
  output <= ff_out;
END structural;