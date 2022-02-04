entity nand_gate_tb is
end nand_gate_tb;

architecture Behavioral of nand_gate_tb is
    COMPONENT nand_gate is
        Port (input_a : in bit;
              input_b : in bit;
              output : out bit);
    end COMPONENT;
    
    signal a, b, c : bit;
     
    begin
        uut : nand_gate port map(
          input_a => a,
          input_b => b,
          output => c
        );
        
        stim : process
        begin
            a <= '0';
            b <= '0';
            wait for 10 ns;
            assert (c = '1')
            report "test failed for combination 00" severity failure;
        
        
            a <= '0';
            b <= '1';
            wait for 10 ns;
            assert (c = '1')
            report "test failed for combination 01" severity failure;
            
            a <= '1';
            b <= '1';
            wait for 10ns;
            assert (c = '0')
            report "test failed for combination 11" severity failure;
            
            a <= '1';
            b <= '0';
            wait for 10 ns;
            assert (c = '1')
            report "test failed for combination 10" severity failure;
            wait;
        end process;
end Behavioral;