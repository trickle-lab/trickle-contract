#![no_std]
#![allow(deprecated)] // events().publish is kept on purpose so the backend's topic decoding stays stable

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, token,
    Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub struct Stream {
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub amount: i128,
    pub start: u64,
    pub duration: u64,
    pub withdrawn: i128,
}

#[contracttype]
pub enum DataKey {
    StreamCounter,
    Stream(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InvalidDuration = 2,
    StreamNotFound = 3,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Create a new payment stream. Sender must authorize this call.
    /// Funds are pulled from sender into the contract immediately, then
    /// released to recipient linearly over `duration` seconds.
    /// Fails with InvalidAmount if amount <= 0 and InvalidDuration if duration == 0.
    pub fn create_stream(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        amount: i128,
        duration: u64,
    ) -> u64 {
        sender.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }
        if duration == 0 {
            panic_with_error!(&env, Error::InvalidDuration);
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&sender, &env.current_contract_address(), &amount);

        let id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::StreamCounter)
            .unwrap_or(0);

        let stream = Stream {
            sender,
            recipient,
            token,
            amount,
            start: env.ledger().timestamp(),
            duration,
            withdrawn: 0,
        };

        env.storage().persistent().set(&DataKey::Stream(id), &stream);
        env.storage().instance().set(&DataKey::StreamCounter, &(id + 1));
        env.events().publish(
            (symbol_short!("stream"), symbol_short!("created")),
            (id, amount, duration),
        );
        id
    }

    /// Returns the amount currently available to withdraw (vested minus already withdrawn).
    pub fn balance(env: Env, stream_id: u64) -> i128 {
        let stream = Self::load_stream(&env, stream_id);
        Self::available(&env, &stream)
    }

    /// Recipient withdraws whatever has vested so far.
    pub fn withdraw(env: Env, stream_id: u64) -> i128 {
        let mut stream = Self::load_stream(&env, stream_id);

        stream.recipient.require_auth();

        let available = Self::available(&env, &stream);
        if available > 0 {
            let token_client = token::Client::new(&env, &stream.token);
            token_client.transfer(
                &env.current_contract_address(),
                &stream.recipient,
                &available,
            );
            stream.withdrawn += available;
            env.storage().persistent().set(&DataKey::Stream(stream_id), &stream);
            env.events().publish(
                (symbol_short!("stream"), symbol_short!("withdraw")),
                (stream_id, available),
            );
        }

        available
    }

    /// Sender cancels the stream: the recipient gets whatever has vested, the sender gets the rest back.
    /// Emits stream/cancel with (stream_id, paid_to_recipient, refunded_to_sender).
    pub fn cancel(env: Env, stream_id: u64) {
        let stream = Self::load_stream(&env, stream_id);

        stream.sender.require_auth();

        let token_client = token::Client::new(&env, &stream.token);
        let vested = Self::available(&env, &stream);
        let remainder = stream.amount - stream.withdrawn - vested;

        if vested > 0 {
            token_client.transfer(&env.current_contract_address(), &stream.recipient, &vested);
        }
        if remainder > 0 {
            token_client.transfer(&env.current_contract_address(), &stream.sender, &remainder);
        }

        env.storage().persistent().remove(&DataKey::Stream(stream_id));
        env.events().publish(
            (symbol_short!("stream"), symbol_short!("cancel")),
            (stream_id, vested, remainder),
        );
    }

    fn load_stream(env: &Env, stream_id: u64) -> Stream {
        env.storage()
            .persistent()
            .get(&DataKey::Stream(stream_id))
            .unwrap_or_else(|| panic_with_error!(env, Error::StreamNotFound))
    }

    fn available(env: &Env, stream: &Stream) -> i128 {
        let now = env.ledger().timestamp();
        let elapsed = if now > stream.start {
            (now - stream.start).min(stream.duration)
        } else {
            0
        };

        let vested = if stream.duration == 0 {
            stream.amount
        } else {
            (stream.amount * elapsed as i128) / stream.duration as i128
        };

        vested - stream.withdrawn
    }
}

mod test;
