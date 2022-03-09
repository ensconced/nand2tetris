
--  Xilinx Single Port Byte-Write Read First RAM
--  This code implements a parameterizable single-port byte-write read-first memory where when data
--  is written to the memory, the output reflects the prior contents of the memory location.
--  If the output data is not needed during writes or the last read value is desired to be
--  retained, it is suggested to use Single Port.Byte-write Enable.No Change Mode template as it is more power efficient.
--  If a reset or enable is not necessary, it may be tied off or removed from the code.
--  Modify the parameters for the desired RAM characteristics.

-- Following libraries have to be used
--use ieee.std_logic_1164.all;
--use std.textio.all;
--use ieee.numeric_std.all;

--Insert the following in the architecture before the begin keyword

--Insert the following in the architecture after the begin keyword