vault program

Purpose
-- user interacts with the contract to store their lamports inside the contract 

Instructions Required
-- create_vault
-- submit_lamports
-- withdraw_lamports
-- close_vault

Accounts Required for 

-- create_vault 
    -- Signer, the one who will pay for the PDA to be created
    -- PDA to derive a new vault for the user, PDA should be a SystemProgram to store lamports

-- submit_lamports
    -- user account (probably be an Account type)
    -- PDA of the vault (SystemAccount)

-- withdraw_lamports
    -- user account (Account type)
    -- PDA of the vault (SystemAccount)

-- close_vault
    -- user account (Account type)
    -- PDA of the vault (SystemAccount)
