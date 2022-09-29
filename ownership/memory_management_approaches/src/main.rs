/*
    Memory Managament Approaches

        Explicit Allocation and Deallocation

            Programmer is responsible for memory management
            C / C++  (malloc(),free(),new)

            Advantage => Programmer has a lot of control on memory
            Disadvantage => Error Prone
                            Memory Leak
                            Invalid Memory Access


        Garbage Collector

            Garbage collector automatically cleans up memory.Memory management is abstracted away from programmer
            Java,Python,Javascript,C#,Go,Ruby

            Advantage => Easy
                         Programmer doesn't need to worry about memory management so they can write faster
            Disadvantage => Wasteful of memory. We also need to install garbage collector with out program
                            It can at inconvenient times and we can't know when. It slows down our program

        Rust is bringing a new approach to memory management that enables us to write more safe and efficient code but since it is very new concept it requires new, different mindset than other languages.
*/
