// switch on a single LED
const machineCode = [
  "0000 0000 0000 0001", // load 1 into A
  "1110 110000 010 000", // load A into D
  "0100 0000 0000 0000", // load 16384 into A
  "1110 0011 0000 1000", // load D into M[A]
  "0000 0000 0000 0000", // load 0 into A
  "1110 0000 0000 0111", // jump
].map((word) => word.replace(/\s/g, ""));

function nextWord() {
  const instruction = machineCode.shift();
  return instruction ? instruction.replace(/\s/g, "") : "0000000000000000";
}

function romLiteral() {
  const result = [];
  for (let i = 0; i < wordCount; i++) {
    result.push(`${i} => "${nextWord()}"`);
  }
  return result.join(", ");
}

function vhdl() {
  return `LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY rom IS
  PORT (
    address : IN STD_ULOGIC_VECTOR(${power - 1} DOWNTO 0);
    data_out : OUT STD_ULOGIC_VECTOR(${wordSize - 1} DOWNTO 0));
END ENTITY;

ARCHITECTURE behavioural OF rom IS
  TYPE ROM_type IS ARRAY (0 TO ${wordCount - 1}) OF STD_ULOGIC_VECTOR(${
    wordSize - 1
  } DOWNTO 0);
  CONSTANT ROM : ROM_type := (${romLiteral()});
BEGIN
  data_out <= ROM(to_integer(unsigned(address)));
END ARCHITECTURE;`;
}

const power = 15;
const wordSize = 16;
const wordCount = 2 ** power;
console.log(vhdl());
