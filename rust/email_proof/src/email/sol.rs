use crate::email::Email;

mod private {
    use alloy_sol_types::sol;

    sol!(#![sol(all_derives)] "../../contracts/src/EmailProof.sol");
}

pub use private::Email as SolEmail;

impl From<Email> for SolEmail {
    fn from(email: Email) -> SolEmail {
        SolEmail {
            from: email.from.unwrap_or_default(),
            to: email.to.unwrap_or_default(),
            subject: email.subject.unwrap_or_default(),
            date: email.date.map(|d| d.timestamp()).unwrap_or_default() as u64,
            body: email.body,
        }
    }
}
