ENTITY nand_gate IS
  PORT (
    input_a : IN BIT;
    input_b : IN BIT;
    output : OUT BIT);
END nand_gate;

ARCHITECTURE Behavioral OF nand_gate IS
BEGIN
  output <= input_a NAND input_b;
END Behavioral;