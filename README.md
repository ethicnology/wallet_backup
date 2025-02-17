# Wallet backup

## Abstract

This document specifies a format for exporting wallet backup data, including 
accounts, descriptors, associated keys, labels, transactions, and partially 
signed Bitcoin transactions (PSBTs). The format aims to standardize wallet 
backup and restore operations across different Bitcoin wallet implementations.

## Motivation

Bitcoin wallets store various forms of metadata beyond just private keys, 
including account structures, descriptors, labels, and transaction history. 
While BIP39 and BIP32 provide standardized mechanisms for key recovery, they 
do not preserve additional wallet-specific metadata necessary for seamless 
migration between wallets.

A standardized wallet backup format ensures that users can migrate wallets 
without losing critical metadata, reducing vendor lock-in and enabling more 
robust recovery options.

## Rationale

Several wallet implementations store backup data in proprietary formats, 
making migration difficult. This proposal aims to introduce a structured, 
human-readable format, leveraging JSON to store all essential wallet metadata 
in a portable way. The JSON Lines format is chosen for compatibility with 
existing tools and for incremental parsing capabilities.

## Specification

A wallet backup is a UTF-8 encoded text file containing a single valid 
JSON object representing the backup structure. This object includes wallet-level 
metadata, multiple accounts, and associated key data.

### Wallet Backup Structure

- `name`: Optional wallet name
- `accounts`: Array of account objects
- `proprietary`: JSON object storing wallet-specific metadata

### Account Object Structure

- `name`: Optional account name
- `descriptor`: Output descriptor defining the account structure
- `timestamp`: Optional timestamp indicating account creation
- `keys`: Dictionary of descriptor keys mapped to metadata
- `labels`: Labels for transactions, addresses, and keys following BIP-0329
- `transactions`: List of fully signed transactions
- `psbts`: List of partially signed Bitcoin transactions
- `proprietary`: JSON object storing account-specific metadata

### Key Object Structure

- `key`: Public key
- `alias`: Optional user-defined alias
- `role`: Defines the role of the key in wallet operations
- `key_type`: Defines the ownership type of the key

### Key Roles

- `main`: Key used for normal spending conditions
- `recovery`: Key designated for recovery scenarios
- `inheritance`: Key to inherit funds if the primary user disappears
- `cosigning`: Key designated for policy-enforcing cosigning

### Key Types

- `internal`: Main user-owned key
- `external`: Heirs or trusted individuals
- `third_party`: Keys held by a service provider

## Importing

- Importing wallets should preserve all metadata they support and 
discard unsupported fields.
- Wallets should warn users if essential data cannot be restored.
- Wallets should ensure that key roles and types are properly mapped 
to their internal structures.

## Security Considerations

- The backup format does not include private keys to avoid unintended 
key exposure.
- Users should encrypt backups at rest to prevent unauthorized access.
- Care should be taken to ensure that proprietary metadata does not 
contain sensitive information.

## Backwards Compatibility

This format is extensible and allows future additions without breaking 
compatibility. Wallets may choose to ignore fields they do not recognize 
while maintaining structural integrity of the backup.

