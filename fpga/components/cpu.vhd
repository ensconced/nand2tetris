LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY cpu IS
  PORT (
    inM : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    instruction : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    reset : IN STD_ULOGIC;
    clock : IN STD_ULOGIC;
    outM : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    addressM : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    writeM : OUT STD_ULOGIC;
    pc : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END cpu;

ARCHITECTURE structural OF cpu IS
  COMPONENT mux16
    PORT (
      input_a : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_b : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      sel : IN STD_ULOGIC;
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT alu
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
  END COMPONENT;
  COMPONENT counter
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      inc : IN STD_ULOGIC;
      load : IN STD_ULOGIC;
      reset : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  COMPONENT register16
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  COMPONENT and_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT not_gate
    PORT (
      input : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT or_gate
    PORT (
      input_a : IN STD_ULOGIC;
      input_b : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  COMPONENT jump_loader
    PORT (
      j1 : IN STD_ULOGIC;
      j2 : IN STD_ULOGIC;
      j3 : IN STD_ULOGIC;
      is_zero : IN STD_ULOGIC;
      is_negative : IN STD_ULOGIC;
      is_c_instruction : IN STD_ULOGIC;
      output : OUT STD_ULOGIC);
  END COMPONENT;
  SIGNAL input_x, input_y, alu_output, reg_a_in, reg_a_out, not_alu_output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL is_c_instruction, is_a_instruction, alu_out_into_reg_a, load_reg_a, load_reg_d, alu_output_is_zero, const_one, jump_loader_out : STD_ULOGIC;
BEGIN
  const_one <= '1';
  alu_a : alu PORT MAP(
    input_x => input_x,
    input_y => input_y,
    zero_x => instruction(10),
    zero_y => instruction(9),
    not_x => instruction(11),
    not_y => instruction(8),
    use_add => instruction(7),
    should_not_output => instruction(6),
    output => alu_output,
    not_output => not_alu_output,
    output_is_zero => alu_output_is_zero
  );
  reg_a : register16 PORT MAP(
    input => reg_a_in,
    output => reg_a_out,
    load => load_reg_a,
    clock => clock
  );
  reg_d : register16 PORT MAP(
    input => alu_output,
    output => input_x,
    load => load_reg_d,
    clock => clock
  );
  and_a : and_gate PORT MAP(
    input_a => is_c_instruction,
    input_b => instruction(4),
    output => load_reg_d
  );
  not_a : not_gate PORT MAP(
    input => is_c_instruction,
    output => is_a_instruction
  );
  and_b : and_gate PORT MAP(
    input_a => is_c_instruction,
    input_b => instruction(5),
    output => alu_out_into_reg_a
  );
  or_a : or_gate PORT MAP(
    input_a => alu_out_into_reg_a,
    input_b => is_a_instruction,
    output => load_reg_a
  );
  mux_a : mux16 PORT MAP(
    input_a => instruction,
    input_b => alu_output,
    sel => alu_out_into_reg_a,
    output => reg_a_in
  );
  mux_b : mux16 PORT MAP(
    input_a => reg_a_out,
    input_b => inM,
    sel => instruction(12),
    output => input_y
  );
  counter_a : counter PORT MAP(
    input => reg_a_out,
    output => pc,
    inc => const_one,
    load => jump_loader_out,
    reset => reset,
    clock => clock
  );
  jump_loader_a : jump_loader PORT MAP(
    j1 => instruction(2),
    j2 => instruction(1),
    j3 => instruction(0),
    is_zero => alu_output_is_zero,
    is_negative => alu_output(0),
    is_c_instruction => instruction(15),
    output => jump_loader_out
  );
  and_c : and_gate PORT MAP(
    input_a => instruction(3),
    input_b => is_c_instruction,
    output => writeM
  );
  is_c_instruction <= instruction(15);
  addressM <= reg_a_out;
  outM <= alu_output;
END structural;