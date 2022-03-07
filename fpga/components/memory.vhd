LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY memory IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(14 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    led_output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END memory;

ARCHITECTURE structural OF memory IS
  COMPONENT ram16k
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(13 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  COMPONENT mux16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT dmux IS
    PORT (
      input : IN STD_ULOGIC;
      sel : IN STD_ULOGIC;
      output_a : OUT STD_ULOGIC;
      output_b : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT register16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  SIGNAL load_leds, load_ram : STD_ULOGIC;
  SIGNAL mux_input_a, mux_input_b, led_reg_out : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  ram : ram16k PORT MAP
    (input => input, address => address(13 DOWNTO 0), output => mux_input_a, load => load_ram, clock => clock);
  mux : mux16 PORT MAP(input_a => mux_input_a, input_b => led_reg_out, sel => address(14), output => output);
  dmux_a : dmux PORT MAP(input => load, sel => address(14), output_a => load_ram, output_b => load_leds);
  led_reg : register16 PORT MAP(input => input, output => led_reg_out, load => load_leds, clock => clock);
  led_output <= led_reg_out;
END structural;