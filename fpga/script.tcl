# create_project -part xc7a35tcpg236-1 -in_memory nand2tetris

read_vhdl components/nand_gate.vhd
read_vhdl components/or_gate.vhd
read_vhdl test/nand_gate_tb.vhd
read_vhdl test/or_gate_tb.vhd

read_xdc constraints.xdc

create_fileset -simset sim_nand_gate
add_files -fileset sim_nand_gate test/nand_gate_tb.vhd
set_property top nand_gate_tb [get_fileset sim_nand_gate]

create_fileset -simset sim_or_gate
add_files -fileset sim_or_gate test/nand_gate_tb.vhd
set_property top or_gate_tb [get_fileset sim_or_gate]

save_project_as -force nand2tetris

launch_simulation -simset sim_nand_gate
launch_simulation -simset sim_or_gate

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
