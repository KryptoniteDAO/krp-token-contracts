use crate::state::{
    read_all_period_config, read_config, read_period_config, read_user_period_config, Config,
    PeriodConfig, UserPeriodConfig,
};
use cosmwasm_std::{Addr, Deps, StdResult};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = read_config(deps.storage)?;
    Ok(config)
}

pub fn query_period_config(deps: Deps, period_id: u64) -> StdResult<PeriodConfig> {
    let period_config = read_period_config(deps.storage, &period_id)?;
    Ok(period_config)
}

pub fn query_user_period_config(deps: Deps, user_address: Addr) -> StdResult<UserPeriodConfig> {
    let user_period_config = read_user_period_config(deps.storage, &user_address)?;
    Ok(user_period_config)
}

pub fn query_all_period_configs(deps: Deps) -> StdResult<Vec<PeriodConfig>> {
    let period_configs = read_all_period_config(deps.storage)?;
    Ok(period_configs)
}
