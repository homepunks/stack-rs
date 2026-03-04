use crate::stack;

pub fn run_tests() {
    {
        let st = stack::DinoStack::<i32>::new();
        assert_eq!(st.len(), 0);
        assert_eq!(st.cap(), 0);
        assert!(st.peek().is_none());
        println!("[+] new_stack_is_empty");
    }

    {
        let mut st = stack::DinoStack::new();
        st.push(42);
        assert_eq!(st.len(), 1);
        assert_eq!(st.peek(), Some(&42));
        println!("[+] push_one");
    }

    {
        let mut st = stack::DinoStack::new();
        st.push(1);
        st.push(2);
        st.push(3);
        assert_eq!(st.pop(), Some(3));
        assert_eq!(st.pop(), Some(2));
        assert_eq!(st.pop(), Some(1));
        assert_eq!(st.pop(), None);
        println!("[+] pop_returns_lifo_order");
    }

    {
        let mut st = stack::DinoStack::new();
        st.push(99);
        assert_eq!(st.peek(), Some(&99));
        assert_eq!(st.peek(), Some(&99));
        assert_eq!(st.len(), 1);
        println!("[+] peek_does_not_remove");
    }

    {
        let mut st = stack::DinoStack::new();
        for i in 0..10 {
            st.push(i);
        }
        assert_eq!(st.len(), 10);
        assert!(st.cap() >= 10);
        println!("[+] grows_correctly");        
    }

    {
        let mut st = stack::DinoStack::<i32>::new();
        assert_eq!(st.pop(), None);
        println!("[+] pop_empty_is_none");
    }

    {
        let mut st = stack::DinoStack::new();
        assert_eq!(st.cap(), 0);

        st.push(1);
        assert_eq!(st.cap(), 1);

        st.push(2);
        assert_eq!(st.cap(), 2);

        st.push(3);
        assert_eq!(st.cap(), 4);

        st.push(4);
        assert_eq!(st.cap(), 4);

        st.push(5);
        assert_eq!(st.cap(), 8);

        println!("[+] cap_grows_correctly");
    }

    println!("\n[+] all tests passed!");
}
