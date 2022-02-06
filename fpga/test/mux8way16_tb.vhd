LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY mux8way16_tb IS
END mux8way16_tb;

ARCHITECTURE Behavioral OF mux8way16_tb IS
  COMPONENT mux8way16 IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(127 DOWNTO 0);
      sel : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
    );
  END COMPONENT;

  SIGNAL input_a, input_b, input_c, input_d, input_e, input_f, input_g, input_h : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL sel : STD_ULOGIC_VECTOR(2 DOWNTO 0);
  SIGNAL output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  uut : mux8way16 PORT MAP(
    input(127 DOWNTO 112) => input_h,
    input(111 DOWNTO 96) => input_g,
    input(95 DOWNTO 80) => input_f,
    input(79 DOWNTO 64) => input_e,
    input(63 DOWNTO 48) => input_d,
    input(47 DOWNTO 32) => input_c,
    input(31 DOWNTO 16) => input_b,
    input(15 DOWNTO 0) => input_a,
    sel => sel,
    output => output
  );
  stim : PROCESS
    VARIABLE val_for_input_a : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "0000000000000000";
    VARIABLE val_for_input_b : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "1010101010101010";
    VARIABLE val_for_input_c : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "0101010101010101";
    VARIABLE val_for_input_d : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "1111111111111111";
    VARIABLE val_for_input_e : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "0000000011111111";
    VARIABLE val_for_input_f : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "1111111100000000";
    VARIABLE val_for_input_g : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "1010101011111111";
    VARIABLE val_for_input_h : STD_ULOGIC_VECTOR(15 DOWNTO 0) := "1111111110101010";
  BEGIN
    input_a <= val_for_input_a;
    input_b <= val_for_input_b;
    input_c <= val_for_input_c;
    input_d <= val_for_input_d;
    input_e <= val_for_input_e;
    input_f <= val_for_input_f;
    input_g <= val_for_input_g;
    input_h <= val_for_input_h;
    sel <= "000";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_a) REPORT "test failed for sel 000" SEVERITY failure;
    sel <= "001";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_b) REPORT "test failed for sel 001" SEVERITY failure;
    sel <= "010";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_c) REPORT "test failed for sel 010" SEVERITY failure;
    sel <= "011";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_d) REPORT "test failed for sel 011" SEVERITY failure;
    sel <= "100";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_e) REPORT "test failed for sel 100" SEVERITY failure;
    sel <= "101";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_f) REPORT "test failed for sel 101" SEVERITY failure;
    sel <= "110";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_g) REPORT "test failed for sel 110" SEVERITY failure;
    sel <= "111";
    WAIT FOR 10 ns;
    ASSERT(output = val_for_input_h) REPORT "test failed for sel 111" SEVERITY failure;
    WAIT;
  END PROCESS;
END Behavioral;