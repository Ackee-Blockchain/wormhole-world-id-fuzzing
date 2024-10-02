use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;

use solana_world_id_onchain_template::instructions::trident_fuzz_verify_and_execute_snapshot::VerifyAndExecuteAlias;

use crate::constants;
use crate::fuzz_instructions::{FuzzAccounts, VerifyAndExecute, VerifyAndExecuteArgs};

type VerifyAndExecuteSnapshot<'info> = VerifyAndExecuteAlias<'info>;

impl<'info> IxOps<'info> for VerifyAndExecute {
    type IxData = solana_world_id_onchain_template::instruction::VerifyAndExecute;
    type IxAccounts = FuzzAccounts;
    type IxSnapshot = VerifyAndExecuteSnapshot<'info>;
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        solana_world_id_onchain_template::ID
    }
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data: VerifyAndExecuteArgs = VerifyAndExecuteArgs {
            root_hash: constants::quardian_set_5_mock::ROOT_HASH,
            nullifier_hash: constants::quardian_set_5_mock::NULLIFIER_HASH,
            proof: constants::quardian_set_5_mock::PROOF,
        };

        let data =
            solana_world_id_onchain_template::instruction::VerifyAndExecute { args: data.into() };
        Ok(data)
    }
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let payer = fuzz_accounts.externalpayer.get_or_create_account(
            self.accounts.payer,
            client,
            58 * LAMPORTS_PER_SOL,
        );

        let root = match fuzz_accounts.root.get(self.accounts.root) {
            Some(r) => r.pubkey(),
            None => Pubkey::default(),
        };

        let config = match fuzz_accounts.config.get(self.accounts.config) {
            Some(c) => c.pubkey(),
            None => Pubkey::default(),
        };

        let nullifier = fuzz_accounts
            .nullifier
            .get_or_create_account(
                self.accounts.nullifier,
                &[
                    b"nullifier",
                    constants::quardian_set_5_mock::NULLIFIER_HASH.as_ref(),
                ],
                &solana_world_id_onchain_template::ID,
            )
            .unwrap();

        let signers = vec![payer.clone()];
        let acc_meta = solana_world_id_onchain_template::accounts::VerifyAndExecute {
            payer: payer.pubkey(),
            root,
            config,
            recipient: constants::quardian_set_5_mock::RECIPIENT,
            nullifier: nullifier.pubkey(),
            world_id_program: solana_world_id_program::ID,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
