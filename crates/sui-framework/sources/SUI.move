// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<SUI> is the token used to pay for gas in Sui
module sui::SUI {
    use sui::Coin;
    use sui::Coin::TreasuryCap;
    use sui::tx_context::TxContext;

    friend sui::genesis;

    /// Name of the coin
    struct SUI has drop {}

    /// Register the token to acquire its `TreasuryCap`.
    /// This should be called only once during genesis creation.
    public(friend) fun new(ctx: &mut TxContext): TreasuryCap<SUI> {
        Coin::create_currency(SUI{}, ctx)
    }

    /// Transfer to a recipient
    public entry fun transfer(c: Coin::Coin<SUI>, recipient: address) {
        Coin::transfer(c, recipient)
    }

}
