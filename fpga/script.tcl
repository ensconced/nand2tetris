foreach source_file [glob components/* test/*.vhd] {
  read_vhdl $source_file
}

read_xdc constraints.xdc

set simset_list {}

foreach test_file [glob test/*.vhd] {
  set base_name [file rootname [file tail $test_file]]
  set new_simset [create_fileset -simset $base_name]
  add_files -fileset $base_name $test_file
  set_property top $base_name $new_simset
  lappend simset_list $new_simset
}

save_project_as -force nand2tetris

foreach simset $simset_list {
  launch_simulation -simset $simset
}

# synth_design -top is_non_zero -part xc7a35tcpg236-1
# opt_design
# place_design
# route_design

# write_bitstream -force out.bit
# open_hw_manager
# connect_hw_server
# open_hw_target
# set_property PROGRAM.FILE out.bit [current_hw_device]
# program_hw_device
