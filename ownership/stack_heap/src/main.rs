    /*
        Before we dive into the Rust's ownership system, we should talk about stack,heap and some memory management approaches.

        https://www.youtube.com/watch?v=_8-ht2AKyH4&t=1s
        https://www.guru99.com/stack-vs-heap.html
        https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap
    */

    /*
        The Memory Assignment for a Program in Typical Architecture

            Heap(Dynamic Memory) => Dynamically grow and shrink variables

            Stack => Function calls and local variables

            Static/Global => Global variables ( Accessible for whole lifetime of a program)

            Code => Instructions

        Stack,global,code fields cannot grow or shrink while program is running.

        When I have heard this, I was a little bit confused about it.If the stack is fixed-size how can we call other functions?
        Actually when we pushing elements onto the stack, we are overwriting to the top of the stack

        Let's say size of our stack is 5. Before our program executes all elements are set to default value which is 0.
        stack => [0,0,0,0,0]

        And we have a "top" pointer which points out the next available position in our stack and it always points the top of the stack

        top_pointer = 0

        When we push elements to the stack, that's happening

        stack.push(5) => stack[top_pointer] = 5 && top_pointer += 1

        stack = [5,0,0,0,0]  top_pointer = 1

        So, we are not adding new elements to the stack, we are just changing the values in the stack by pushing and popping. Stack remainx the same size

        Now, I will introduce you an concept you might heard about it.

        stack = [1, 2, 3, 4, 0]  top_pointer = 4

        stack.push(5)

        stack = [1, 2, 3, 4, 5] top_pointer = 5

        What if we wanted add another element to the stack ? Let's try it

        stack.push(6)

        We want to add another element but our limit is 5 and we have reached it.
        So, this will throw STACK OVERFLOW error
        STACK OVERFLOW => Basically, we have exceeded the stack size
    */

    /*
        STACK

        Stack is used to store local variables and function calls.
        All variables that are stored on the stack must have known,fixed size

        Stack is very fast since Big O of popping and pushing is O(1).
        There are one location we can add or remove element, the top
    */

    /*
        HEAP

        Heap is used to store dynamically grow or shrink variables
        Heap is less organized than stack. There is no absolute location for a variable.
        When we want to store a variable in the heap, available location from the heap is allocated four this variable and the address of the variable is pushed onto the stack

        We can't access the data on the heap directly.
        We store the actual data on the heap and the address of the data on the stack
        When we want to access this data, first we go to the stack and read the address and go that address on the heap.
        Since the address of a variable is fixed-size we can store it on the stack
        We have to follow a pointer accessing data from the heap, so it is slower than stack
    */
