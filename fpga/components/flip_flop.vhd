LIBRARY ieee;
USE ieee.std_logic_1164.ALL;

ENTITY flip_flop IS
  PORT (
    input : IN STD_ULOGIC;
    output : OUT STD_ULOGIC;
    clock : IN STD_ULOGIC);
END flip_flop;

ARCHITECTURE dff OF flip_flop IS
BEGIN
  PROCESS (clock)
  BEGIN
    IF (rising_edge(clock)) THEN
      output <= input;
    END IF;
  END PROCESS;
END dff;