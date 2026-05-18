#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String,
};

#[derive(Clone)]
#[contracttype]
pub struct Certificate {
    pub cert_id: String,
    pub student_wallet: Address,
    pub student_name: String,
    pub title: String,
    pub doc_hash: String,
    pub issuer: Address,
    pub issued_at: u64,
    pub revoked: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Certificate(String),
}

#[contract]
pub struct CertiPassContract;

#[contractimpl]
impl CertiPassContract {
    // 1. Inisialisasi admin pertama kali
    // Untuk versi demo, auth dimatikan agar mudah dites di Stellar Lab.
    pub fn initialize(env: Env, admin: Address) -> bool {
        let existing_admin: Option<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Admin);

        if existing_admin.is_some() {
            return false;
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);

        true
    }

    // 2. Cek apakah admin sudah pernah diset
    pub fn has_admin(env: Env) -> bool {
        let admin: Option<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Admin);

        admin.is_some()
    }

    // 3. Ambil data admin
    // Kalau belum initialize, hasilnya None/null, bukan error.
    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
    }

    // 4. Terbitkan sertifikat / ijazah digital
    pub fn issue_cert(
        env: Env,
        cert_id: String,
        student_wallet: Address,
        student_name: String,
        title: String,
        doc_hash: String,
    ) -> bool {
        let issuer: Option<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Admin);

        if issuer.is_none() {
            return false;
        }

        let issuer_address = issuer.unwrap();

        let key = DataKey::Certificate(cert_id.clone());

        let existing_cert: Option<Certificate> = env
            .storage()
            .instance()
            .get(&key);

        if existing_cert.is_some() {
            return false;
        }

        let certificate = Certificate {
            cert_id,
            student_wallet,
            student_name,
            title,
            doc_hash,
            issuer: issuer_address,
            issued_at: env.ledger().timestamp(),
            revoked: false,
        };

        env.storage()
            .instance()
            .set(&key, &certificate);

        true
    }

    // 5. Verifikasi sertifikat berdasarkan cert_id dan doc_hash
    pub fn verify_cert(
        env: Env,
        cert_id: String,
        doc_hash: String,
    ) -> bool {
        let cert: Option<Certificate> = env
            .storage()
            .instance()
            .get(&DataKey::Certificate(cert_id));

        match cert {
            Some(certificate) => {
                certificate.doc_hash == doc_hash && certificate.revoked == false
            }
            None => false,
        }
    }

    // 6. Ambil detail sertifikat
    // Kalau tidak ada, hasilnya None/null, bukan error.
    pub fn get_cert(
        env: Env,
        cert_id: String,
    ) -> Option<Certificate> {
        env.storage()
            .instance()
            .get(&DataKey::Certificate(cert_id))
    }

    // 7. Cabut / revoke sertifikat
    pub fn revoke_cert(
        env: Env,
        cert_id: String,
    ) -> bool {
        let admin: Option<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Admin);

        if admin.is_none() {
            return false;
        }

        let key = DataKey::Certificate(cert_id.clone());

        let cert: Option<Certificate> = env
            .storage()
            .instance()
            .get(&key);

        if cert.is_none() {
            return false;
        }

        let mut certificate = cert.unwrap();

        if certificate.revoked {
            return false;
        }

        certificate.revoked = true;

        env.storage()
            .instance()
            .set(&key, &certificate);

        true
    }

    // 8. Cek status revoke
    pub fn is_revoked(
        env: Env,
        cert_id: String,
    ) -> bool {
        let cert: Option<Certificate> = env
            .storage()
            .instance()
            .get(&DataKey::Certificate(cert_id));

        match cert {
            Some(certificate) => certificate.revoked,
            None => false,
        }
    }

    // 9. Cek apakah sertifikat ada
    pub fn cert_exists(
        env: Env,
        cert_id: String,
    ) -> bool {
        let cert: Option<Certificate> = env
            .storage()
            .instance()
            .get(&DataKey::Certificate(cert_id));

        cert.is_some()
    }

    // 10. Ganti admin
    // Versi demo: selama admin sudah ada, admin bisa diganti.
    pub fn change_admin(
        env: Env,
        new_admin: Address,
    ) -> bool {
        let existing_admin: Option<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Admin);

        if existing_admin.is_none() {
            return false;
        }

        env.storage()
            .instance()
            .set(&DataKey::Admin, &new_admin);

        true
    }
}