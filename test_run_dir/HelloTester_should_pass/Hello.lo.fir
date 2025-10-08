FIRRTL version 1.2.0
circuit Hello :
  module DataMemory :
    input clock : Clock
    input reset : UInt<1>
    input io_address : UInt<16> @[src/main/scala/DataMemory.scala 4:14]
    output io_dataRead : UInt<32> @[src/main/scala/DataMemory.scala 4:14]
    input io_writeEnable : UInt<1> @[src/main/scala/DataMemory.scala 4:14]
    input io_dataWrite : UInt<32> @[src/main/scala/DataMemory.scala 4:14]
    input io_testerEnable : UInt<1> @[src/main/scala/DataMemory.scala 4:14]
    input io_testerAddress : UInt<16> @[src/main/scala/DataMemory.scala 4:14]
    output io_testerDataRead : UInt<32> @[src/main/scala/DataMemory.scala 4:14]
    input io_testerWriteEnable : UInt<1> @[src/main/scala/DataMemory.scala 4:14]
    input io_testerDataWrite : UInt<32> @[src/main/scala/DataMemory.scala 4:14]

    mem memory : @[src/main/scala/DataMemory.scala 18:20]
      data-type => UInt<32>
      depth => 65536
      read-latency => 0
      write-latency => 1
      reader => io_testerDataRead_MPORT
      reader => io_dataRead_MPORT
      writer => MPORT
      writer => MPORT_1
      read-under-write => undefined
    node _GEN_0 = validif(io_testerWriteEnable, io_testerAddress) @[src/main/scala/DataMemory.scala 24:32]
    node _GEN_1 = validif(io_testerWriteEnable, clock) @[src/main/scala/DataMemory.scala 24:32]
    node _GEN_2 = mux(io_testerWriteEnable, UInt<1>("h1"), UInt<1>("h0")) @[src/main/scala/DataMemory.scala 18:20 24:32]
    node _GEN_3 = validif(io_testerWriteEnable, UInt<1>("h1")) @[src/main/scala/DataMemory.scala 24:32]
    node _GEN_4 = validif(io_testerWriteEnable, io_testerDataWrite) @[src/main/scala/DataMemory.scala 24:32]
    node _GEN_5 = mux(io_testerWriteEnable, io_testerDataWrite, memory.io_testerDataRead_MPORT.data) @[src/main/scala/DataMemory.scala 22:23 24:32 26:25]
    node _GEN_6 = validif(io_writeEnable, io_address) @[src/main/scala/DataMemory.scala 32:26]
    node _GEN_7 = validif(io_writeEnable, clock) @[src/main/scala/DataMemory.scala 32:26]
    node _GEN_8 = mux(io_writeEnable, UInt<1>("h1"), UInt<1>("h0")) @[src/main/scala/DataMemory.scala 18:20 32:26]
    node _GEN_9 = validif(io_writeEnable, UInt<1>("h1")) @[src/main/scala/DataMemory.scala 32:26]
    node _GEN_10 = validif(io_writeEnable, io_dataWrite) @[src/main/scala/DataMemory.scala 32:26]
    node _GEN_11 = mux(io_writeEnable, io_dataWrite, memory.io_dataRead_MPORT.data) @[src/main/scala/DataMemory.scala 30:17 32:26 34:19]
    node _GEN_12 = validif(io_testerEnable, io_testerAddress) @[src/main/scala/DataMemory.scala 20:24 22:37]
    node _GEN_13 = validif(io_testerEnable, clock) @[src/main/scala/DataMemory.scala 20:24 22:37]
    node _GEN_14 = mux(io_testerEnable, UInt<1>("h1"), UInt<1>("h0")) @[src/main/scala/DataMemory.scala 18:20 20:24 22:37]
    node _GEN_15 = mux(io_testerEnable, _GEN_5, UInt<32>("h0")) @[src/main/scala/DataMemory.scala 20:24 31:23]
    node _GEN_16 = mux(io_testerEnable, UInt<32>("h0"), _GEN_11) @[src/main/scala/DataMemory.scala 20:24 23:17]
    node _GEN_17 = validif(io_testerEnable, _GEN_0) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_18 = validif(io_testerEnable, _GEN_1) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_19 = mux(io_testerEnable, _GEN_2, UInt<1>("h0")) @[src/main/scala/DataMemory.scala 18:20 20:24]
    node _GEN_20 = validif(io_testerEnable, _GEN_3) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_21 = validif(io_testerEnable, _GEN_4) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_22 = validif(eq(io_testerEnable, UInt<1>("h0")), io_address) @[src/main/scala/DataMemory.scala 20:24 30:31]
    node _GEN_23 = validif(eq(io_testerEnable, UInt<1>("h0")), clock) @[src/main/scala/DataMemory.scala 20:24 30:31]
    node _GEN_24 = mux(io_testerEnable, UInt<1>("h0"), UInt<1>("h1")) @[src/main/scala/DataMemory.scala 18:20 20:24 30:31]
    node _GEN_25 = validif(eq(io_testerEnable, UInt<1>("h0")), _GEN_6) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_26 = validif(eq(io_testerEnable, UInt<1>("h0")), _GEN_7) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_27 = mux(io_testerEnable, UInt<1>("h0"), _GEN_8) @[src/main/scala/DataMemory.scala 18:20 20:24]
    node _GEN_28 = validif(eq(io_testerEnable, UInt<1>("h0")), _GEN_9) @[src/main/scala/DataMemory.scala 20:24]
    node _GEN_29 = validif(eq(io_testerEnable, UInt<1>("h0")), _GEN_10) @[src/main/scala/DataMemory.scala 20:24]
    io_dataRead <= _GEN_16
    io_testerDataRead <= _GEN_15
    memory.io_testerDataRead_MPORT.addr <= _GEN_12
    memory.io_testerDataRead_MPORT.en <= _GEN_14
    memory.io_testerDataRead_MPORT.clk <= _GEN_13
    memory.io_dataRead_MPORT.addr <= _GEN_22
    memory.io_dataRead_MPORT.en <= _GEN_24
    memory.io_dataRead_MPORT.clk <= _GEN_23
    memory.MPORT.addr <= _GEN_17
    memory.MPORT.en <= _GEN_19
    memory.MPORT.clk <= _GEN_18
    memory.MPORT.data <= _GEN_21
    memory.MPORT.mask <= _GEN_20
    memory.MPORT_1.addr <= _GEN_25
    memory.MPORT_1.en <= _GEN_27
    memory.MPORT_1.clk <= _GEN_26
    memory.MPORT_1.data <= _GEN_29
    memory.MPORT_1.mask <= _GEN_28

  module Hello :
    input clock : Clock
    input reset : UInt<1>
    output io_done : UInt<1> @[src/main/scala/Hello.scala 21:14]
    input io_run : UInt<1> @[src/main/scala/Hello.scala 21:14]
    input io_testerDataMemEnable : UInt<1> @[src/main/scala/Hello.scala 21:14]
    input io_testerDataMemAddress : UInt<16> @[src/main/scala/Hello.scala 21:14]
    output io_testerDataMemDataRead : UInt<32> @[src/main/scala/Hello.scala 21:14]
    input io_testerDataMemWriteEnable : UInt<1> @[src/main/scala/Hello.scala 21:14]
    input io_testerDataMemDataWrite : UInt<32> @[src/main/scala/Hello.scala 21:14]

    inst dataMemory of DataMemory @[src/main/scala/Hello.scala 33:26]
    reg addressCounterReg : UInt<17>, clock with :
      reset => (UInt<1>("h0"), addressCounterReg) @[src/main/scala/Hello.scala 35:34]
    reg dataReg : UInt<32>, clock with :
      reset => (UInt<1>("h0"), dataReg) @[src/main/scala/Hello.scala 36:24]
    node _T = eq(addressCounterReg, UInt<16>("h320")) @[src/main/scala/Hello.scala 39:26]
    node _addressCounterReg_T = add(addressCounterReg, UInt<16>("h1")) @[src/main/scala/Hello.scala 44:46]
    node _addressCounterReg_T_1 = tail(_addressCounterReg_T, 1) @[src/main/scala/Hello.scala 44:46]
    node _GEN_0 = mux(io_run, _addressCounterReg_T_1, addressCounterReg) @[src/main/scala/Hello.scala 43:17 44:25 35:34]
    node _GEN_1 = mux(_T, UInt<1>("h1"), UInt<1>("h0")) @[src/main/scala/Hello.scala 38:11 39:42 40:13]
    node _GEN_2 = mux(_T, addressCounterReg, _GEN_0) @[src/main/scala/Hello.scala 39:42 41:23]
    node _T_1 = bits(addressCounterReg, 0, 0) @[src/main/scala/Hello.scala 48:25]
    node _T_2 = eq(_T_1, UInt<1>("h0")) @[src/main/scala/Hello.scala 48:29]
    node _GEN_3 = mux(_T_2, dataMemory.io_dataRead, dataReg) @[src/main/scala/Hello.scala 48:42 49:13 36:24]
    node _invertedPixel_T = bits(dataReg, 7, 0) @[src/main/scala/Hello.scala 53:43]
    node _invertedPixel_T_1 = not(_invertedPixel_T) @[src/main/scala/Hello.scala 53:35]
    node _invertedPixel_T_2 = cat(UInt<24>("h0"), _invertedPixel_T_1) @[src/main/scala/Hello.scala 53:23]
    node _dataMemory_io_address_T = bits(addressCounterReg, 0, 0) @[src/main/scala/Hello.scala 55:49]
    node _dataMemory_io_address_T_1 = bits(addressCounterReg, 16, 1) @[src/main/scala/Hello.scala 55:71]
    node _dataMemory_io_address_T_2 = add(_dataMemory_io_address_T_1, UInt<16>("h190")) @[src/main/scala/Hello.scala 55:77]
    node _dataMemory_io_address_T_3 = tail(_dataMemory_io_address_T_2, 1) @[src/main/scala/Hello.scala 55:77]
    node _dataMemory_io_address_T_4 = bits(addressCounterReg, 16, 1) @[src/main/scala/Hello.scala 55:108]
    node _dataMemory_io_address_T_5 = mux(_dataMemory_io_address_T, _dataMemory_io_address_T_3, _dataMemory_io_address_T_4) @[src/main/scala/Hello.scala 55:31]
    node _dataMemory_io_writeEnable_T = bits(addressCounterReg, 0, 0) @[src/main/scala/Hello.scala 57:49]
    node invertedPixel = _invertedPixel_T_2 @[src/main/scala/Hello.scala 52:27 53:17]
    io_done <= _GEN_1
    io_testerDataMemDataRead <= dataMemory.io_testerDataRead @[src/main/scala/Hello.scala 61:28]
    dataMemory.clock <= clock
    dataMemory.reset <= reset
    dataMemory.io_address <= _dataMemory_io_address_T_5 @[src/main/scala/Hello.scala 55:25]
    dataMemory.io_writeEnable <= _dataMemory_io_writeEnable_T @[src/main/scala/Hello.scala 57:29]
    dataMemory.io_dataWrite <= invertedPixel @[src/main/scala/Hello.scala 56:27]
    dataMemory.io_testerEnable <= io_testerDataMemEnable @[src/main/scala/Hello.scala 63:30]
    dataMemory.io_testerAddress <= io_testerDataMemAddress @[src/main/scala/Hello.scala 60:31]
    dataMemory.io_testerWriteEnable <= io_testerDataMemWriteEnable @[src/main/scala/Hello.scala 64:35]
    dataMemory.io_testerDataWrite <= io_testerDataMemDataWrite @[src/main/scala/Hello.scala 62:33]
    addressCounterReg <= mux(reset, UInt<17>("h0"), _GEN_2) @[src/main/scala/Hello.scala 35:{34,34}]
    dataReg <= mux(reset, UInt<32>("h0"), _GEN_3) @[src/main/scala/Hello.scala 36:{24,24}]
