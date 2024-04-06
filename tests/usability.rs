use terrors::OneOf;

#[derive(Debug)]
struct NotEnoughMemory;

#[derive(Debug)]
struct Timeout;

#[derive(Debug)]
struct RetriesExhausted;

#[test]
fn retry() {
    fn inner() -> Result<(), OneOf<(NotEnoughMemory, RetriesExhausted)>> {
        for _ in 0..3 {
            let Err(err) = does_stuff() else {
                return Ok(());
            };

            match err.narrow::<Timeout, _>() {
                Ok(_timeout) => continue,
                Err(allocation_oneof) => {
                    println!("didn't get Timeout, now trying to get NotEnoughMemory");
                    let allocation_oneof: OneOf<(NotEnoughMemory,)> = allocation_oneof;
                    let allocation = allocation_oneof.narrow::<NotEnoughMemory, _>().unwrap();

                    return Err(OneOf::new(allocation));
                }
            }
        }

        Err(OneOf::new(RetriesExhausted))
    }

    let _ = dbg!(inner());
}

fn does_stuff() -> Result<(), OneOf<(NotEnoughMemory, Timeout)>> {
    // TODO Try impl after superset type work
    let _allocation = match allocates() {
        Ok(a) => a,
        Err(e) => return Err(e.broaden()),
    };

    // TODO Try impl after superset type work
    let _chat = match chats() {
        Ok(c) => c,
        Err(e) => return Err(OneOf::new(e)),
    };

    Ok(())
}

fn allocates() -> Result<(), OneOf<(NotEnoughMemory,)>> {
    let result: Result<(), NotEnoughMemory> = Err(NotEnoughMemory);

    result?;

    Ok(())
}

fn chats() -> Result<(), Timeout> {
    Err(Timeout)
}

#[test]
fn smoke() {
    let o_1: OneOf<(u32, String)> = OneOf::new(5_u32);
    let _narrowed_1: u32 = o_1.narrow::<u32, _>().unwrap();

    let o_2: OneOf<(String, u32)> = OneOf::new(5_u32);
    let _narrowed_2: u32 = o_2.narrow::<u32, _>().unwrap();

    let o_3: OneOf<(String, u32)> = OneOf::new("5".to_string());
    let _narrowed_3: OneOf<(String,)> = o_3.narrow::<u32, _>().unwrap_err();

    let o_4: OneOf<(String, u32)> = OneOf::new("5".to_string());

    let _: String = o_4.narrow().unwrap();

    let o_5: OneOf<(String, u32)> = OneOf::new("5".to_string());
    o_5.narrow::<String, _>().unwrap();

    let o_6: OneOf<(String, u32)> = OneOf::new("5".to_string());
    let o_7: OneOf<(u32, String)> = o_6.broaden();
    let o_8: OneOf<(String, u32)> = o_7.subset().unwrap();
    let _: OneOf<(u32, String)> = o_8.subset().unwrap();

    let o_9: OneOf<(u8, u16, u32)> = OneOf::new(3_u32);
    let _: Result<OneOf<(u16,)>, OneOf<(u8, u32)>> = o_9.subset();
    let o_10: OneOf<(u8, u16, u32)> = OneOf::new(3_u32);
    let _: Result<u16, OneOf<(u8, u32)>> = o_10.narrow();
}

#[test]
fn debug() {
    use std::error::Error;
    use std::io;

    let o_1: OneOf<(u32, String)> = OneOf::new(5_u32);

    // Debug is implemented if all types in the type set implement Debug
    dbg!(&o_1);

    // Display is implemented if all types in the type set implement Display
    println!("{}", o_1);

    type E = io::Error;
    let e = io::Error::new(io::ErrorKind::Other, "wuaaaaahhhzzaaaaaaaa");

    let o_2: OneOf<(E,)> = OneOf::new(e);

    // std::error::Error is implemented if all types in the type set implement it
    dbg!(o_2.source());

    let o_3: OneOf<(u32, String)> = OneOf::new("hey".to_string());
    dbg!(o_3);
}

#[test]
fn multi_match() {
    use terrors::E2;

    let o_1: OneOf<(u32, String)> = OneOf::new(5_u32);

    match o_1.as_enum() {
        E2::A(u) => {
            println!("handling {u}: u32")
        }
        E2::B(s) => {
            println!("handling {s}: String")
        }
    }

    match o_1.to_enum() {
        E2::A(u) => {
            println!("handling {u}: u32")
        }
        E2::B(s) => {
            println!("handling {s}: String")
        }
    }
}

#[test]
fn multi_narrow() {
    use terrors::E2;

    struct Timeout {}
    struct Backoff {}

    let o_1: OneOf<(u8, u16, u32, u64, u128)> = OneOf::new(5_u32);

    let _narrow_res: Result<OneOf<(u8, u128)>, OneOf<(u16, u32, u64)>> = o_1.subset();

    let o_2: OneOf<(u8, u16, Backoff, Timeout, u32, u64, u128)> = OneOf::new(Timeout {});

    match o_2.subset::<(Timeout, Backoff), _>().unwrap().to_enum() {
        E2::A(Timeout {}) => {
            println!(":)");
        }
        E2::B(Backoff {}) => {
            unreachable!()
        }
    }
}
