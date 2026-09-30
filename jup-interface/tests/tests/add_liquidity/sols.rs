use inf1_jup_interface::consts::{INF_LST_LIST_ID, INF_MINT_ADDR};
use jupiter_amm_interface::{FeeMode, QuoteParams, SwapMode};
use test_utils::{KeyedUiAccount, ALL_FIXTURES, CONST_PUBKEYS};

use crate::common::{swap_test, SwapUserAccs};

fn fixtures_accs() -> SwapUserAccs<&'static str> {
    SwapUserAccs::default()
        .with_signer("swsol-token-acc-owner")
        .with_inp_token_acc("swsol-token-acc")
        .with_out_token_acc("inf-token-acc")
}

/// swsol is not yet listed on INF on mainnet, so `swsol-mint`/`swsol-pool` are
/// cloned from mainnet while `swsol-token-acc`, `swsol-reserves`,
/// `swsol-pf-accum` and `sols-calc-state` are synthesized. The swsol entry is
/// added by hand to `lst-state-list` and `flatslab-slab`, and the sols sol
/// value calculator is loaded in as `programs/sols-calc.so`.
#[test]
fn add_liq_swsol_fixture_basic() {
    swap_test(
        INF_LST_LIST_ID,
        QuoteParams {
            amount: 1_000_000_000,
            input_mint: *CONST_PUBKEYS.swsol_mint(),
            output_mint: INF_MINT_ADDR.into(),
            swap_mode: SwapMode::ExactIn,
            fee_mode: FeeMode::Normal,
        },
        &ALL_FIXTURES,
        fixtures_accs().map(|n| KeyedUiAccount::from_test_fixtures_json(n).into_keyed_account()),
    );
}
