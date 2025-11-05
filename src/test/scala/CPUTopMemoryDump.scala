import chisel3._
import chiseltest._
import org.scalatest.flatspec.AnyFlatSpec
import java.util

class CPUTopMemoryDump extends AnyFlatSpec with ChiselScalatestTester {

  "CPUTopMemoryDumpTester" should "pass" in {
    test(new CPUTop())
      .withAnnotations(Seq(WriteVcdAnnotation)) { dut =>
        dut.clock.setTimeout(0)

        // Do not run the CPU
        dut.io.run.poke(false.B)

        // Load the program memory with instructions
        System.out.print("\nLoading the program memory with instructions... ")
        // Uncomment one of the following line depending on the program you want to load to the program memory
        val program = Programs.primes
        // val program = Programs.program2
        for (address <- 0 to program.length - 1) {
          dut.io.testerProgMemEnable.poke(true.B)
          dut.io.testerProgMemWriteEnable.poke(true.B)
          dut.io.testerProgMemAddress.poke(address)
          dut.io.testerProgMemDataWrite.poke(program(address))
          dut.clock.step(1)
        }
        dut.io.testerProgMemEnable.poke(false.B)
        System.out.println("Done!")

        // Run the simulation of the CPU
        System.out.println("\nRun the simulation of the CPU")
        // Start the CPU
        dut.io.run.poke(true.B)
        var running = true
        var maxInstructions =
          20000 // Will terminate early, but manage to find 31 primes
        var instructionsCounter = maxInstructions
        while (running) {
          System.out.print(
            "\rRunning cycle: " + (maxInstructions - instructionsCounter)
          )
          dut.clock.step(1)
          instructionsCounter = instructionsCounter - 1
          running =
            dut.io.done.peekBoolean() == false && instructionsCounter > 0
        }
        dut.io.run.poke(false.B)
        System.out.println(" - Done!")

        // Dump the data memory content
        System.out.print("\nDump the data memory content... ")
        System.out.println("Done!")

        System.out.println("\n=== MEMORY DUMP ===")
        System.out.println("addresses 0-399:")
        for (i <- 0 to 100) {
          dut.io.testerDataMemEnable.poke(true.B)
          dut.io.testerDataMemWriteEnable.poke(false.B)
          dut.io.testerDataMemAddress.poke(i)
          val data = dut.io.testerDataMemDataRead.peekInt().toInt
          System.out.println(f"Address: $i%3d  Value: $data%d")
          dut.clock.step(1)
        }

        dut.io.testerDataMemEnable.poke(false.B)

        System.out.println("\nEnd of simulation")

      }
  }
}
