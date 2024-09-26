use crate::dkim::static_resolver::StaticResolver;
use mail_auth::dkim::verify::DkimVerifier;
use mail_auth::{AuthenticatedMessage, DkimOutput, DkimResult, Error as AuthError};

pub fn verify_dkim(email: &[u8], dns_records: &[String]) -> Result<(), AuthError> {
    let authenticated_message = AuthenticatedMessage::parse(email).ok_or(AuthError::ParseError)?;
    let dns_record = dns_records.first().ok_or(AuthError::UnsupportedKeyType)?;
    let resolver = StaticResolver::new(dns_record);

    let future = DkimVerifier::verify_dkim(&resolver, &authenticated_message);
    let dkim_outputs = futures::executor::block_on(future);

    assert_dkim_verified(dkim_outputs)
}

fn assert_dkim_verified(verification_output: Vec<DkimOutput>) -> Result<(), AuthError> {
    if verification_output.is_empty() {
        return Err(AuthError::NoHeadersFound);
    }

    for output in verification_output {
        match output.result().clone() {
            DkimResult::None => return Err(AuthError::NoHeadersFound),
            DkimResult::Fail(err)
            | DkimResult::Neutral(err)
            | DkimResult::PermError(err)
            | DkimResult::TempError(err) => return Err(err),
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_dkim_verified {
        use super::*;

        #[test]
        fn test_returns_err_if_no_headers_found() {
            let verification_output = vec![];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Err(AuthError::NoHeadersFound));
        }

        #[test]
        fn test_all_ok_single_header() {
            let verification_output = vec![DkimOutput::pass()];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Ok(()));
        }

        #[test]
        fn test_all_ok_multiple_headers() {
            let verification_output = vec![DkimOutput::pass(), DkimOutput::pass()];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Ok(()));
        }

        #[test]
        fn test_returns_err_if_fail() {
            let verification_output =
                vec![DkimOutput::pass(), DkimOutput::fail(AuthError::MissingParameters)];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Err(AuthError::MissingParameters));
        }

        #[test]
        fn test_returns_err_if_perm_error() {
            let verification_output =
                vec![DkimOutput::pass(), DkimOutput::perm_err(AuthError::MissingParameters)];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Err(AuthError::MissingParameters));
        }

        #[test]
        fn test_returns_err_if_temp_error() {
            let verification_output =
                vec![DkimOutput::pass(), DkimOutput::temp_err(AuthError::MissingParameters)];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Err(AuthError::MissingParameters));
        }

        #[test]
        fn test_returns_err_if_neutral() {
            let verification_output =
                vec![DkimOutput::pass(), DkimOutput::neutral(AuthError::MissingParameters)];
            let result = assert_dkim_verified(verification_output);
            assert_eq!(result, Err(AuthError::MissingParameters));
        }
    }
}
