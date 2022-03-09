LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE std.textio.ALL;
USE ieee.numeric_std.ALL;

ENTITY ram16k IS
  PORT (
    input : IN STD_ULOGIC_VECTOR(15 DOWNTO 0);
    address : IN STD_ULOGIC_VECTOR(13 DOWNTO 0);
    output : OUT STD_ULOGIC_VECTOR(15 DOWNTO 0);
    load : IN STD_ULOGIC;
    clock : IN STD_ULOGIC);
END ram16k;

ARCHITECTURE structural OF ram16k IS
  CONSTANT C_INIT_FILE : STRING := "just use zeroes"; -- Specify name/location of RAM initialization file if using one (leave blank if not)
  TYPE ram_type IS ARRAY (16383 DOWNTO 0) OF STD_ULOGIC_VECTOR (15 DOWNTO 0); -- 2D Array Declaration for RAM signal
  FUNCTION initramfromfile (ramfilename : IN STRING) RETURN ram_type IS
    FILE ramfile : text IS IN ramfilename;
    VARIABLE ramfileline : line;
    VARIABLE ram_name : ram_type;
    VARIABLE bitvec : bit_vector(15 DOWNTO 0);
  BEGIN
    FOR i IN ram_type'RANGE LOOP
      readline (ramfile, ramfileline);
      read (ramfileline, bitvec);
      ram_name(i) := to_stdlogicvector(bitvec);
    END LOOP;
    RETURN ram_name;
  END FUNCTION;

  FUNCTION init_from_file_or_zeroes(ramfile : STRING) RETURN ram_type IS
  BEGIN
    IF ramfile = "<Init File Name>" THEN
      RETURN InitRamFromFile("<Init File Name>");
    ELSE
      RETURN (OTHERS => (OTHERS => '0'));
    END IF;
  END;
  SIGNAL ram_name : ram_type := init_from_file_or_zeroes(C_INIT_FILE);
BEGIN
  PROCESS (clock)
  BEGIN
    IF (clock 'event AND clock = '1') THEN
      IF (load = '1') THEN
        ram_name (to_integer(unsigned(address))) <= input;
      END IF;
    END IF;
  END PROCESS;
  output <= ram_name (to_integer(unsigned(address)));
END structural;