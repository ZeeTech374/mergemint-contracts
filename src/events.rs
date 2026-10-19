use soroban_sdk::{Address, BytesN, Env, Symbol};

pub fn emit_bounty_created(env: &Env, bounty_id: &BytesN<32>, creator: &Address, reward: &i128) {
    let topic = Symbol::new(env, "bounty_created");
    env.events().publish(
        (topic, creator.clone()),
        (bounty_id.clone(), *reward),
    );
}

pub fn emit_bounty_claimed(env: &Env, bounty_id: &BytesN<32>, contributor: &Address) {
    let topic = Symbol::new(env, "bounty_claimed");
    env.events().publish(
        (topic, contributor.clone()),
        bounty_id.clone(),
    );
}

pub fn emit_bounty_completed(env: &Env, bounty_id: &BytesN<32>, contributor: &Address) {
    let topic = Symbol::new(env, "bounty_completed");
    env.events().publish(
        (topic, contributor.clone()),
        bounty_id.clone(),
    );
}

pub fn emit_reward_paid(env: &Env, bounty_id: &BytesN<32>, contributor: &Address, amount: &i128) {
    let topic = Symbol::new(env, "reward_paid");
    env.events().publish(
        (topic, contributor.clone()),
        (bounty_id.clone(), *amount),
    );
}

pub fn emit_admin_initialized(env: &Env, admin: &Address) {
    let topic = Symbol::new(env, "admin_initialized");
    env.events().publish((topic,), admin.clone());
}

pub fn emit_min_reward_amount_updated(env: &Env, token: &Address, amount: &i128) {
    let topic = Symbol::new(env, "min_reward_amount_updated");
    env.events()
        .publish((topic, token.clone()), *amount);
}

pub fn emit_bounty_ownership_transferred(
    env: &Env,
    bounty_id: &BytesN<32>,
    previous_owner: &Address,
    new_owner: &Address,
) {
    let topic = Symbol::new(env, "bounty_ownership_transferred");
    env.events().publish(
        (topic, bounty_id.clone()),
        (previous_owner.clone(), new_owner.clone()),
    );
}
