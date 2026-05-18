# CertiPass DApp

CertiPass DApp is a blockchain-based digital certificate verification system built on Stellar Soroban. This project allows an issuer, such as a university, organization, or event committee, to publish certificate data on-chain so anyone can verify its authenticity without contacting the issuer manually.

## Project Description

Fake certificates and forged academic documents are still common problems in education, recruitment, and professional training. Traditional certificate verification usually requires manual confirmation from the issuing institution, which can take time and is difficult to scale.

CertiPass solves this problem by storing certificate identity and document hash on the Stellar blockchain through a Soroban smart contract. The actual certificate file does not need to be stored directly on-chain. Instead, the system stores a unique certificate ID, certificate owner, title, issuer address, document hash, issue timestamp, and revocation status.

With this system, a company, campus, or organization can verify whether a certificate is authentic by checking the certificate ID and matching document hash. If the certificate is revoked, the smart contract will mark it as invalid.

## Project Vision

The vision of CertiPass is to create a simple, transparent, and tamper-resistant certificate verification system for academic and professional use.

This project aims to:

- Reduce certificate and diploma forgery.
- Make certificate verification faster and more transparent.
- Provide a blockchain-based trust layer for education and training documents.
- Help institutions issue verifiable digital credentials.
- Demonstrate a real-world use case of Stellar Soroban beyond token transfer.

## Key Features

### 1. Admin Initialization

The contract can be initialized with an admin wallet address. The admin represents the certificate issuer, such as a university, workshop organizer, or institution.

Function:

```text
initialize(admin)
```

### 2. Issue Digital Certificate

The issuer can create a new certificate by storing the certificate ID, student wallet, student name, certificate title, document hash, issuer, timestamp, and revoked status.

Function:

```text
issue_cert(cert_id, student_wallet, student_name, title, doc_hash)
```

### 3. Verify Certificate

Anyone can verify a certificate by providing the certificate ID and document hash. The smart contract will return `true` if the certificate exists, the hash matches, and the certificate has not been revoked.

Function:

```text
verify_cert(cert_id, doc_hash)
```

### 4. Get Certificate Details

Users can retrieve certificate information stored in the smart contract.

Function:

```text
get_cert(cert_id)
```

### 5. Revoke Certificate

If a certificate is invalid, expired, or issued by mistake, the issuer can revoke it. After revocation, the certificate will no longer pass verification.

Function:

```text
revoke_cert(cert_id)
```

### 6. Check Revocation Status

Users can check whether a certificate has been revoked.

Function:

```text
is_revoked(cert_id)
```

### 7. Check Certificate Existence

Users can check whether a certificate ID already exists in the smart contract.

Function:

```text
cert_exists(cert_id)
```

### 8. Change Admin

The contract allows the admin address to be changed when needed.

Function:

```text
change_admin(new_admin)
```

## Smart Contract Details

```text
Smart Contract Name : CertiPassContract
Network             : Stellar Testnet
Contract Type       : Soroban Smart Contract
Language            : Rust
```

## Deployed Smart Contract Details

```text
CONTRACT ID:
CCSQGKZZWKHUETR7SG2NQEVDZQNG73KEC7JNNGHV3I7RONCBYQMBCKPW

NETWORK:
Stellar Testnet
```

> Note: Replace `CCSQGKZZWKHUETR7SG2NQEVDZQNG73KEC7JNNGHV3I7RONCBYQMBCKPW` with the latest deployed contract ID from Stellar Lab after uploading and deploying the final WASM file.

## Tech Stack

- Stellar Testnet
- Soroban Smart Contract
- Rust
- Stellar Lab
- Soroban SDK

## Contract Data Structure

Each certificate stores the following data:

```text
cert_id        : Unique certificate ID
student_wallet : Wallet address of certificate owner
student_name   : Name of certificate owner
title          : Certificate title
doc_hash       : Hash or unique fingerprint of certificate document
issuer         : Wallet address of certificate issuer
issued_at      : Blockchain ledger timestamp
revoked        : Certificate revocation status
```

## Example Demo Data

### Initialize Admin

```text
admin:
GAIJUKMRGPTNQS5EMOW55KCIHGWDQEIEWFST3H5NDP3QA3BTA6DL7XHV
```

### Issue Certificate

```text
cert_id:
CERT-UNDIP-STELLAR-001

student_wallet:
GAIJUKMRGPTNQS5EMOW55KCIHGWDQEIEWFST3H5NDP3QA3BTA6DL7XHV

student_name:
Galih Sudaryono

title:
Build on Blockchain with Stellar Bootcamp

doc_hash:
hash_sertifikat_galih_001
```

### Verify Certificate

```text
cert_id:
CERT-UNDIP-STELLAR-001

doc_hash:
hash_sertifikat_galih_001
```

Expected result:

```text
true
```

### Verify Using Wrong Hash

```text
cert_id:
CERT-UNDIP-STELLAR-001

doc_hash:
hash_palsu_123
```

Expected result:

```text
false
```

### Revoke Certificate

```text
cert_id:
CERT-UNDIP-STELLAR-001
```

After revocation, verifying the original certificate will return:

```text
false
```

## How the System Works

```text
1. Issuer initializes the smart contract with an admin wallet.
2. Issuer creates a certificate by storing certificate data and document hash on-chain.
3. User receives a certificate ID and document hash.
4. Verifier checks the certificate using the certificate ID and document hash.
5. Smart contract returns true if the certificate is valid.
6. If the certificate is revoked, verification returns false.
```

## Demo Flow

```text
initialize(admin)
        ↓
get_admin()
        ↓
issue_cert(cert_id, student_wallet, student_name, title, doc_hash)
        ↓
cert_exists(cert_id)
        ↓
get_cert(cert_id)
        ↓
verify_cert(cert_id, doc_hash)
        ↓
revoke_cert(cert_id)
        ↓
verify_cert(cert_id, doc_hash)
```

## Why Blockchain?

Blockchain is suitable for this project because certificate verification requires trust, transparency, and data integrity.

CertiPass uses Stellar Soroban to make certificate records:

- Transparent
- Tamper-resistant
- Publicly verifiable
- Independent from centralized database manipulation
- Easy to audit through the blockchain network

Instead of trusting a private database, verifiers can check the certificate status directly through the deployed smart contract.

## Future Development

Future improvements may include:

- QR code-based certificate verification.
- IPFS integration for off-chain certificate metadata.
- NFT-based certificate ownership.
- Web dashboard for issuers and verifiers.
- Multi-institution issuer support.
- Role-based access control for admin and university staff.

## Submission Summary

CertiPass DApp is a Stellar Soroban-based digital certificate verification system. It allows institutions to issue certificate records on-chain and enables public verification using certificate ID and document hash. The project demonstrates how blockchain can be used to reduce certificate fraud and improve trust in academic or professional credentials.