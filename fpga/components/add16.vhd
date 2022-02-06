LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY add16 IS
  PORT (
    input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
END add16;

ARCHITECTURE structural OF add16 IS
  COMPONENT half_adder
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
  END COMPONENT;
  COMPONENT full_adder
    PORT (
      input : IN STD_ULOGIC_VECTOR(2 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(1 DOWNTO 0));
  END COMPONENT;
  SIGNAL carries : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  adder_a : half_adder PORT MAP(input_a => input_a(0), input_b => input_b(0), output(0) => output(0), output(1) => carries(0));
  adder_b : full_adder PORT MAP(input(0) => carries(0), input(1) => input_a(1), input(2) => input_b(1), output(0) => output(1), output(1) => carries(1));
  adder_c : full_adder PORT MAP(input(0) => carries(1), input(1) => input_a(2), input(2) => input_b(2), output(0) => output(2), output(1) => carries(2));
  adder_d : full_adder PORT MAP(input(0) => carries(2), input(1) => input_a(3), input(2) => input_b(3), output(0) => output(3), output(1) => carries(3));
  adder_e : full_adder PORT MAP(input(0) => carries(3), input(1) => input_a(4), input(2) => input_b(4), output(0) => output(4), output(1) => carries(4));
  adder_f : full_adder PORT MAP(input(0) => carries(4), input(1) => input_a(5), input(2) => input_b(5), output(0) => output(5), output(1) => carries(5));
  adder_g : full_adder PORT MAP(input(0) => carries(5), input(1) => input_a(6), input(2) => input_b(6), output(0) => output(6), output(1) => carries(6));
  adder_h : full_adder PORT MAP(input(0) => carries(6), input(1) => input_a(7), input(2) => input_b(7), output(0) => output(7), output(1) => carries(7));
  adder_i : full_adder PORT MAP(input(0) => carries(7), input(1) => input_a(8), input(2) => input_b(8), output(0) => output(8), output(1) => carries(8));
  adder_j : full_adder PORT MAP(input(0) => carries(8), input(1) => input_a(9), input(2) => input_b(9), output(0) => output(9), output(1) => carries(9));
  adder_k : full_adder PORT MAP(input(0) => carries(9), input(1) => input_a(10), input(2) => input_b(10), output(0) => output(10), output(1) => carries(10));
  adder_l : full_adder PORT MAP(input(0) => carries(10), input(1) => input_a(11), input(2) => input_b(11), output(0) => output(11), output(1) => carries(11));
  adder_m : full_adder PORT MAP(input(0) => carries(11), input(1) => input_a(12), input(2) => input_b(12), output(0) => output(12), output(1) => carries(12));
  adder_n : full_adder PORT MAP(input(0) => carries(12), input(1) => input_a(13), input(2) => input_b(13), output(0) => output(13), output(1) => carries(13));
  adder_o : full_adder PORT MAP(input(0) => carries(13), input(1) => input_a(14), input(2) => input_b(14), output(0) => output(14), output(1) => carries(14));
  adder_p : full_adder PORT MAP(input(0) => carries(14), input(1) => input_a(15), input(2) => input_b(15), output(0) => output(15), output(1) => carries(15));
END structural;