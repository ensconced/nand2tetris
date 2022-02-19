function randomBit() {
  return Math.random() < 0.5 ? "0" : "1";
}

function randomWord() {
  const result = [];
  for (let i = 0; i < wordSize; i++) {
    result.push(randomBit());
  }
  return result.join("");
}

function romLiteral() {
  const result = [];
  for (let i = 0; i < wordCount; i++) {
    result.push(`${i} => "${randomWord()}"`);
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
