use snafu::ResultExt;

use cosmwasm::errors::{Result, SerializeErr, Unauthorized};
use cosmwasm::serde::to_vec;
use cosmwasm::traits::{Api, Extern, Storage};
use cosmwasm::types::{Response, Env};

use crate::msg::{ComputeRichestResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{millionaires, millionaires_read, State};

pub fn init<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    msg: InitMsg,
) -> Result<Response> {
    Ok(Response::default())
}

pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    env: Env,
    msg: HandleMsg,
) -> Result<Response> {
    match msg {
        HandleMsg::AddMillionaire { address, net_worth} => try_add_millionaire(deps, env, address, net_worth),
    }
}

pub fn try_add_millionaire<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    _env: Env,
    _address: HumanAddress,
    _net_worth: u8
) -> Result<Response> {
    let millionaire = Millionaire {
        address: _address,
        net_worth: _net_worth,
    };
    millionaires(&mut deps.storage).save(&millionaire)?;
    Ok(millionaire);
}

pub fn query<S: Storage, A: Api>(deps: &Extern<S, A>, msg: QueryMsg) -> Result<HumanAddress> {
    match msg {
        QueryMsg::ComputeRichest {} => compute_richest(deps),
    }
}

fn compute_richest<S: Storage, A: Api>(deps: &Extern<S, A>) -> Result<HumanAddress> {
    let millionaire = millionaires_read(&deps.storage).load()?;

    let resp = ComputeRichestResponse { address: millionaire.address };
    to_vec(&resp).context(SerializeErr {
        kind: "ComputeRichestResponse",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm::errors::Error;
    use cosmwasm::mock::{dependencies, mock_env};
    use cosmwasm::serde::from_slice;
    use cosmwasm::types::coin;

    #[test]
    fn proper_initialization() {
        let mut deps = dependencies(20);

        let msg = InitMsg { };
        let env = mock_env(&deps.api, "creator", &coin("1000000", "uscrt"), &[]);

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    // fred: enigma1rm7ljr05l38k72ftszer9ull9jwx9vd5re8ey4 100scrt
    // thief: enigma1xgpjnwg7xtr3s6h7tk3vlknuzqpcpj2749nayf 1000scrt

    #[test]
    fn add_millionaires() {
        let mut deps = dependencies(20);

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
        let mut deps = dependencies(20);

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
        let res = query(&deps, QueryMsg::ComputeRichest {}).unwrap();
        let value: ComputeRichestResponse = from_slice(&res).unwrap();
        assert_eq!(thief, value.address);
    }

}
