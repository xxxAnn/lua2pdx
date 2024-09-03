use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::rc::Rc;

use log::debug;
use log::trace;
use piccolo::Function;
use piccolo::StashedClosure;
use piccolo::{Callback, CallbackReturn, Closure, Executor, Lua, StaticError, Table};

use env_logger;

fn main() -> Result<(), StaticError> {

    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("Usage: pdxlua <filename> [level]");
        return Ok(());
    }

    let code = std::fs::read_to_string(args.get(1).unwrap()).unwrap();


    env_logger::builder()
    .filter_level(match args.get(2) {
        Some(level) => level.parse().unwrap_or(log::LevelFilter::Error),
        _ => log::LevelFilter::Error
    })
    .init();

    let mut lua = Lua::empty();

    let game_loaded = Rc::new(RefCell::new(HashMap::<String, StashedClosure>::new()));

    let lua_game_loaded = game_loaded.clone();
    let lua_game_loaded_bis = game_loaded.clone();

    lua.enter(move |ctx| {
        let pdxlua = Table::new(&ctx);

        let _ = pdxlua.set(
            ctx,
            "register_scripted_effect",
            Callback::from_fn(&ctx, move |ctx, _, mut stack| {
                let (s, f) = stack.consume::<(String, Closure)>(ctx)?;
                let x = ctx.stash(f);
                lua_game_loaded.borrow_mut().insert(s, x);

                Ok(CallbackReturn::Return)
            }),
        );
        
        let _ = ctx.set_global("pdxlua", pdxlua);
    });

    let exec = lua.try_enter(|ctx| {
        use std::io::Cursor;
        
        let closure = Closure::load(ctx, None, Cursor::new(code))?;
        Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;

    let _ = lua.execute::<>(&exec)?;


    for (s, closure) in lua_game_loaded_bis.borrow().iter() {
        let run_generator = lua.try_enter(|ctx| {
            Ok(ctx.stash(Executor::start(ctx, ctx.fetch(closure).into(), ("some_text = { ".to_string()))))
        })?;

        let v = lua.execute::<String>(&run_generator)?;

        debug!("Name: {}, Value: {}", s, v);
    }


    
    Ok(())


    //println!("Hello, world!");
}
