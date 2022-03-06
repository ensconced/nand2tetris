LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY computer IS
  PORT (
    reset : IN STD_ULOGIC;
    clock : IN STD_ULOGIC;
    led_output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0)
  );
END computer;

ARCHITECTURE structural OF computer IS
  COMPONENT rom
    PORT (
      address : IN STD_ULOGIC_VECTOR(14 DOWNTO 0);
      data_out : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0));
  END COMPONENT;
  COMPONENT cpu
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
  END COMPONENT;
  COMPONENT memory IS
    PORT (
      input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
      address : IN STD_ULOGIC_VECTOR(14 DOWNTO 0);
      output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      led_output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
      load : IN STD_ULOGIC;
      clock : IN STD_ULOGIC);
  END COMPONENT;
  SIGNAL writeM : STD_ULOGIC;
  SIGNAL inM, instruction, outM, addressM, pc : STD_ULOGIC_VECTOR(15 DOWNTO 0);
BEGIN
  rom_a : rom PORT MAP
    (address => pc(14 DOWNTO 0), data_out => instruction);
  cpu_a : cpu PORT MAP
    (inM => inM, instruction => instruction, reset => reset, clock => clock, outM => outM, addressM => addressM, writeM => writeM, pc => pc);
  memory_a : memory PORT MAP(input => outM, address => addressM(14 DOWNTO 0), output => inM, led_output => led_output, load => writeM, clock => clock);
END structural;