LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY alu_tb IS
END alu_tb;

ARCHITECTURE Behavioral OF alu_tb IS
  COMPONENT alu IS
    PORT (
      input_x : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      input_y : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      zero_x : IN STD_ULOGIC;
      not_x : IN STD_ULOGIC;
      zero_y : IN STD_ULOGIC;
      not_y : IN STD_ULOGIC;
      use_add : IN STD_ULOGIC;
      should_not_output : IN STD_ULOGIC;
      output, not_output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      output_is_zero : OUT STD_ULOGIC
    );
  END COMPONENT;

  SIGNAL input_x, input_y, output, not_output : STD_ULOGIC_VECTOR(15 DOWNTO 0);
  SIGNAL zero_x, zero_y, not_x, not_y, use_add, should_not_output, output_is_zero : STD_ULOGIC;
BEGIN
  uut : alu PORT MAP(
    input_x => input_x,
    input_y => input_y,
    zero_x => zero_x,
    not_x => not_x,
    zero_y => zero_y,
    not_y => not_y,
    use_add => use_add,
    should_not_output => should_not_output,
    output => output,
    not_output => not_output,
    output_is_zero => output_is_zero
  );
  stim : PROCESS
    TYPE test_inputs IS ARRAY(23 DOWNTO 0) OF signed(15 DOWNTO 0);
    VARIABLE test_nums : test_inputs := (
      to_signed(-32768, 16),
      to_signed(-27493, 16),
      to_signed(-25394, 16),
      to_signed(-21205, 16),
      to_signed(-20158, 16),
      to_signed(-14675, 16),
      to_signed(-11554, 16),
      to_signed(-7618, 16),
      to_signed(-681, 16),
      to_signed(0, 16),
      to_signed(1, 16),
      to_signed(5463, 16),
      to_signed(6203, 16),
      to_signed(6882, 16),
      to_signed(10921, 16),
      to_signed(11361, 16),
      to_signed(13866, 16),
      to_signed(16538, 16),
      to_signed(16663, 16),
      to_signed(17656, 16),
      to_signed(19188, 16),
      to_signed(26990, 16),
      to_signed(27183, 16),
      to_signed(32767, 16)
    );
  BEGIN
    FOR i IN test_nums' RANGE LOOP
      FOR j IN test_nums' RANGE LOOP
        -- zero
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '1';
        zero_y <= '1';
        not_x <= '0';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(0, 16))) REPORT "zero test failed with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j)))SEVERITY failure;
        -- one
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '1';
        zero_y <= '1';
        not_x <= '1';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(to_unsigned(1, 16))) REPORT "one test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j)))SEVERITY failure;
        -- minus 1
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '1';
        zero_y <= '1';
        not_x <= '1';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(to_signed(-1, 16))) REPORT "minus 1 test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j)))SEVERITY failure;
        -- x
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '0';
        zero_y <= '1';
        not_x <= '0';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i))) REPORT "x test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- y
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '1';
        zero_y <= '0';
        not_x <= '0';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(j))) REPORT "y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- not x
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '0';
        zero_y <= '1';
        not_x <= '1';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(NOT test_nums(i))) REPORT "not x test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- not y
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '1';
        zero_y <= '0';
        not_x <= '0';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(NOT test_nums(j))) REPORT "not x test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- minus x
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '0';
        zero_y <= '1';
        not_x <= '0';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(-test_nums(i))) REPORT "minus x test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- minus y
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '1';
        zero_y <= '0';
        not_x <= '1';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(-test_nums(j))) REPORT "minus y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x plus one
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '0';
        zero_y <= '1';
        not_x <= '1';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) + 1)) REPORT "x plus one test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- y plus one
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '1';
        zero_y <= '0';
        not_x <= '1';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(j) + 1)) REPORT "y plus one test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x minus one
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '0';
        zero_y <= '1';
        not_x <= '0';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) - 1)) REPORT "x minus one test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- y minus one
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '1';
        zero_y <= '0';
        not_x <= '1';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(j) - 1)) REPORT "y minus one test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x plus y
        use_add <= '1';
        should_not_output <= '0';
        zero_x <= '0';
        zero_y <= '0';
        not_x <= '0';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) + test_nums(j))) REPORT "x plus y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x minus y
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '0';
        zero_y <= '0';
        not_x <= '1';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) - test_nums(j))) REPORT "x minus y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- y minus x
        use_add <= '1';
        should_not_output <= '1';
        zero_x <= '0';
        zero_y <= '0';
        not_x <= '0';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(j) - test_nums(i))) REPORT "y minus x test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x and y
        use_add <= '0';
        should_not_output <= '0';
        zero_x <= '0';
        zero_y <= '0';
        not_x <= '0';
        not_y <= '0';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) AND test_nums(j))) REPORT "x and y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
        -- x or y
        use_add <= '0';
        should_not_output <= '1';
        zero_x <= '0';
        zero_y <= '0';
        not_x <= '1';
        not_y <= '1';
        input_x <= STD_ULOGIC_VECTOR(test_nums(i));
        input_y <= STD_ULOGIC_VECTOR(test_nums(j));
        WAIT FOR 10 ns;
        ASSERT (output = STD_ULOGIC_VECTOR(test_nums(i) OR test_nums(j))) REPORT "x or y test failed for with input_x: " &
        INTEGER'image(to_integer(test_nums(i))) & " input_y: " & INTEGER'image(to_integer(test_nums(j))) SEVERITY failure;
      END LOOP;
    END LOOP;
    WAIT;
  END PROCESS;
END Behavioral;