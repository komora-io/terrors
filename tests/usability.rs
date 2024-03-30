use terrors::OneOf;

#[derive(Debug)]
struct NotEnoughMemory;

#[derive(Debug)]
struct Timeout;

#[derive(Debug)]
struct RetriesExhausted;

fn retry() -> Result<(), OneOf<(NotEnoughMemory, RetriesExhausted)>> {
    for _ in 0..3 {
        let err = if let Err(e) = does_stuff() {
            e
        } else {
            return Ok(());
        };

        match err.narrow::<Timeout, (NotEnoughMemory,), _>() {
            Ok(_timeout) => continue,
            Err(allocation_oneof) => {
                // TODO make broadening work
                // return Err(OneOf::new(allocation))
                let allocation_oneof: OneOf<(NotEnoughMemory,)> = allocation_oneof;
                let allocation = allocation_oneof.narrow::<NotEnoughMemory, (), _>().unwrap();

                return Err(OneOf::new(allocation));
            }
        }
    }

    Err(OneOf::new(RetriesExhausted))
}

fn does_stuff() -> Result<(), OneOf<(NotEnoughMemory, Timeout)>> {
    // TODO Try impl after superset type work
    let _allocation = match allocates() {
        Ok(a) => a,
        Err(e) => return Err(OneOf::new(e)),
    };

    // TODO Try impl after superset type work
    let _chat = match chats() {
        Ok(c) => c,
        Err(e) => return Err(OneOf::new(e)),
    };

    Ok(())
}

fn allocates() -> Result<(), NotEnoughMemory> {
    Err(NotEnoughMemory)
}

fn chats() -> Result<(), Timeout> {
    Err(Timeout)
}

#[test]
fn smoke() {
    let o_1: OneOf<(u32, String)> = OneOf::new(5_u32);
    let narrowed_1 = o_1.narrow::<u32, (String,), _>();

    let o_2: OneOf<(String, u32)> = OneOf::new(5_u32);
    let narrowed_2 = o_2.narrow::<u32, (String,), _>();

    let o_3: OneOf<(String, u32)> = OneOf::new(5_u32);
    let narrowed_3 = o_3.narrow::<u32, (String,), _>();

    let o_4: OneOf<(String, u32)> = OneOf::new(5_u32);
    let narrowed_3 = o_4.narrow::<u32, (String,), _>();

    retry();
}
