// Memory Management are 5 JARGONS
    /*  1.Mutable 
        2.Heap and Memory
        3.Ownership
        4.References and Borrowing
        5.Lifetimes

     */
     1) Mutable :
        By default the rust takes mutable as var and it hepls the processes in multithreading or Synchroniation
        so that it can be confirm that we no need to track that var 
        Thread Safe
        No Synchronzation is need so proccess can be done concurently

    (2) Stack V/S Heap Memory Allocation 

        Stack is used for the primitive data type (which are allocated at compile time)

        Heap on string,Vector like runtime 

        fn main(){
            let x :i8 = 3;
            println!("{}",x);  ---> stack they **USED TO STORE IN THE STACK FRAME WORK ONE FUNCTION ONE LAYER AND DYANAMIC ARE POINTED TO HEAD FIRST HEAD ADDRESS
**

            let str: String = String::form("Hello");--->head
            println!("{}",str);

        }

           
