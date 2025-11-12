use std::{collections::HashMap, path::Path};

use mollusk_svm::{result::InstructionResult, Mollusk};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

use crate::{test_fixtures_dir, CONST_PUBKEYS, FIXTURE_PROGRAMS};

/// A mollusk instance with following programs all loaded in:
/// - all programs in test-fixtures/programs (NB: subdirs excluded)
/// - spl token program
/// - associated token program
pub fn mollusk_inf_fixture_ctl() -> Mollusk {
    let mut svm = mollusk_with_token_progs();
    let paths = FIXTURE_PROGRAMS.into_iter().map(|(fname, key)| {
        (
            test_fixtures_dir()
                .join("programs")
                .join(fname)
                .with_extension("so"),
            key,
        )
    });
    mollusk_add_so_files(&mut svm, paths);
    svm
}

fn mollusk_with_token_progs() -> Mollusk {
    let mut res = Mollusk::default();
    mollusk_svm_programs_token::token::add_program(&mut res);
    mollusk_svm_programs_token::associated_token::add_program(&mut res);
    res
}

/// All programs have owner = BPF_LOADER_UPGRADEABLE
fn mollusk_add_so_files(
    svm: &mut Mollusk,
    so_files: impl IntoIterator<Item = (impl AsRef<Path>, Pubkey)>,
) {
    so_files.into_iter().for_each(|(path, key)| {
        svm.add_program_with_elf_and_loader(
            &key,
            &std::fs::read(path).unwrap(),
            CONST_PUBKEYS.bpf_loader_upgradeable(),
        );
    });
}

/// Returns `(accounts before, exec result)`
pub fn mollusk_exec(
    svm: &Mollusk,
    ix: &Instruction,
    onchain_state: &HashMap<Pubkey, Account>,
) -> (Vec<(Pubkey, Account)>, InstructionResult) {
    let mut keys: Vec<_> = ix.accounts.iter().map(|a| a.pubkey).collect();
    keys.sort_unstable();
    keys.dedup();

    let accs_bef: Vec<_> = keys
        .iter()
        .map(|k| {
            let (k, v) = onchain_state.get_key_value(k).unwrap();
            (*k, v.clone())
        })
        .collect();

    let res = svm.process_instruction(ix, &accs_bef);

    (accs_bef, res)
}
