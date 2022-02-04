read_vhdl components/nand_gate.vhd
read_vhdl components/or_gate.vhd
read_xdc constraints.xdc
synth_design -top or_gate -part xc7a35tcpg236-1 -verbose
opt_design
place_design
route_design
write_bitstream -force out.bit
open_hw_manager
connect_hw_server
open_hw_target
set_property PROGRAM.FILE out.bit [current_hw_device]
program_hw_device
