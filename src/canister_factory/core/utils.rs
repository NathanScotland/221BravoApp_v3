use super::runtime::RUNTIME_STATE;
use candid::Principal;
use ic_cdk::api::call::RejectionCode;

pub fn log(text: impl AsRef<str>){
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.add_log(text.as_ref().to_string())
    });
}

pub async fn canister_call<T, U> (canister: &str, method: &str, args: T, cycles: Option<u128>) -> Result<(U,), (RejectionCode, String)> 
where
    T: candid::CandidType,
    U: for<'a> candid::Deserialize<'a, > + candid::CandidType,
{
    // for T: use your struct directly
    // for U: use result - Result<(YourResponseType,), (ic_cdk::api::call::RejectionCode, String)>
    if cycles == None {
        // Call without CYCLES
        let canister_id = Principal::from_text(canister);
        match canister_id {
            Ok(pr) => {
                let call:Result<(U,), (RejectionCode, String)> = 
                ic_cdk::call(pr, method, ( args,)).await;
                match call {
                    Ok(value) => { Ok(value)}
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            Err(e) => { 
                let er = format!("Could not parse canister principal {:?}", e);
                return Err((ic_cdk::api::call::RejectionCode::Unknown, er))
            }
        }
    } else {
        // Call with CYCLES
        let canister_id = Principal::from_text(canister);
        match canister_id {
            Ok(pr) => {
                let call:Result<(U,), (RejectionCode, String)> = 
                ic_cdk::api::call::call_with_payment128(pr, method, ( args,), cycles.unwrap()).await;
                match call {
                    Ok(value) => { Ok(value)}
                    Err(e) => {
                        Err(e)
                    }
                }
            },
            Err(e) => { 
                let er = format!("Could not parse canister principal {:?}", e);
                return Err((ic_cdk::api::call::RejectionCode::Unknown, er))
            }
        }
    }

}

pub async fn get_self_cycle_balance() -> u64 {
    let self_id: String = RUNTIME_STATE.with(|s|{
        s.borrow().data.get_self_id()
    });
    let self_cycles_result: Result<(u64,), (ic_cdk::api::call::RejectionCode, String)> = canister_call(
        self_id.as_str(),
        "get_cycles_balance",
        (),
        None
    ).await;
    match self_cycles_result {
        Ok(v) => { return v.0 }
        Err(e) => {
            return 0u64;
        }
    }
}
