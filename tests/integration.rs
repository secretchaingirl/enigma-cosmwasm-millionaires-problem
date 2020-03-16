use cosmwasm::mock::mock_env;
use cosmwasm::serde::from_slice;
use cosmwasm::types::{coin, ContractResult};

use cosmwasm_vm::testing::{handle, init, mock_instance, query};

use millionaires::msg::{ComputeRichestResponse, HandleMsg, InitMsg, QueryMsg};

/**
This integration test tries to run and call the generated wasm.
It depends on a release build being available already. You can create that with:

cargo wasm && wasm-gc ./target/wasm32-unknown-unknown/release/enigmawasm-millionaires-problem.wasm

Then running `cargo test` will validate we can properly call into that generated data.

You can easily convert unit tests to integration tests.
1. First copy them over verbatum,
2. Then change
    let mut deps = dependencies(20);
To
    let mut deps = mock_instance(WASM);
3. If you access raw storage, where ever you see something like:
    deps.storage.get(CONFIG_KEY).expect("no data stored");
 replace it with:
    deps.with_storage(|store| {
        let data = store.get(CONFIG_KEY).expect("no data stored");
        //...
    });
4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)
5. When matching on error codes, you can not use Error types, but rather must use strings:
     match res {
         Err(Error::Unauthorized{..}) => {},
         _ => panic!("Must return unauthorized error"),
     }
     becomes:
     match res {
        ContractResult::Err(msg) => assert_eq!(msg, "Unauthorized"),
        _ => panic!("Expected error"),
     }



**/

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/enigmawasm-millionaires-problem.wasm");
// You can uncomment this line instead to test productionified build from cosmwasm-opt
// static WASM: &[u8] = include_bytes!("../contract.wasm");

#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM);

    let msg = InitMsg {  };
    let env = mock_env(&deps.api, "creator", &coin("1000", "scrt"), &[]);

    // we can just call .unwrap() to assert this was a success
    let res = init(&mut deps, env, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn add_millionaires() {
  let mut deps = dependencies(WASM);

  // create instance
  let env = mock_env(
    &deps.api,
    "creator",
          &coin("2", "token"),
          &coin("2", "token"),
  );
  let res = init(&mut deps, env, msg).unwrap();

  // add millionaire "fred"
  let env = mock_env(&deps.api, "fred", &coin("2", "token"), &[]);
  let fred = "enigma1rm7ljr05l38k72ftszer9ull9jwx9vd5re8ey4";

  let msg = HandleMsg::AddMillionaire { address: fred, net_worth: 100000000 };
  let res = handle(&mut deps, env, msg).unwrap();

  // TODO: get each millionaire and verify state
  assert(true);
}

#[test]
fn compute_richest() {
  let mut deps = dependencies(WASM);

  // create instance
  let env = mock_env(
    &deps.api,
    "creator",
          &coin("2", "token"),
          &coin("2", "token"),
  );
  let res = init(&mut deps, env, msg).unwrap();

  // add millionaire "fred"
  let env = mock_env(&deps.api, "fred", &coin("2", "token"), &[]);
  let fred = "enigma1rm7ljr05l38k72ftszer9ull9jwx9vd5re8ey4";

  let msg = HandleMsg::AddMillionaire { address: fred, net_worth: 100000000 };
  let res = handle(&mut deps, env, msg).unwrap();
  
  // add millionaire "thief"
  let thief = "enigma1xgpjnwg7xtr3s6h7tk3vlknuzqpcpj2749nayf";
  let env = mock_env(&deps.api, "thief", &coin("2", "token"), &[]);
  
  let msg = HandleMsg::AddMillionaire { address: thief, net_worth: 1000000000 };
  let res = handle(&mut deps, env, msg).unwrap();

  // should return richest millionaire - thief
  let res = query(mut &deps, QueryMsg::ComputeRichest {}).unwrap();
  let value: ComputeRichestResponse = from_slice(&res).unwrap();
  assert_eq!(thief, value.address);
}
