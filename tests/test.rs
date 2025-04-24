use std::time::Duration;

// simulate slow tests

macro_rules! t {
    ($name:ident) => {
        #[test]
        fn $name() {
            std::thread::sleep(Duration::from_millis(200));
        }
    };
}

t!(dummy);
t!(dummy2);
t!(dummy3);
t!(dummy4);
t!(dummy5);
t!(dummy6);
t!(dummy7);
t!(dummy8);
t!(dummy9);
t!(dummy10);
t!(dummy11);
t!(dummy12);
t!(dummy13);
t!(dummy14);
t!(dummy15);
t!(dummy16);
t!(dummy17);
t!(dummy18);
t!(dummy19);
t!(dummy20);
t!(dummy21);
t!(dummy22);
t!(dummy23);
t!(dummy24);
t!(dummy25);
t!(dummy26);
t!(dummy27);
t!(dummy28);
t!(dummy29);
t!(dummy30);