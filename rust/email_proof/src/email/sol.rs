use crate::email::Email;

mod private {
    use alloy_sol_types::sol;

    sol!(#![sol(all_derives)] "../../contracts/src/EmailProof.sol");
}

pub(crate) use private::Email as SolEmail;

impl From<Email> for SolEmail {
    fn from(email: Email) -> SolEmail {
        SolEmail {
            from: email.from,
            to: email.to,
            subject: email.subject.unwrap_or_default(),
            body: email.body,
        }
    }
}
