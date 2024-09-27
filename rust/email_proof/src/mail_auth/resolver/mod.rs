use std::collections::HashMap;

use domain_key::DomainKey;

pub(crate) mod domain_key;

type Domain = (String, String);

pub struct Resolver {
    domains: HashMap<Domain, DomainKey>,
}

impl Resolver {
    pub fn dkim_key(&self, domain: &str, selector: &str) -> Option<&DomainKey> {
        self.domains.get(&(domain.to_owned(), selector.to_owned()))
    }
}

impl From<Vec<(Domain, DomainKey)>> for Resolver {
    fn from(value: Vec<(Domain, DomainKey)>) -> Self {
        let domains = value.into_iter().collect();
        Self { domains }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref NEWENGLAND_SELECTOR_KEY: DomainKey = "v=DKIM1; \
    p=MIGJAoGBALVI635dLK4cJJAH3Lx6upo3X/Lm1tQz3mezcWTA3BUBnyIsdnRf57aD5BtNmhPrYYDlWlzw3UgnKisIxktkk5+iMQMlFtAS10JB8L3YadXNJY+JBcbeSi5TgJe4WFzNgW95FWDAuSTRXSWZfA/8xjflbTLDx0euFZOM7C4T0GwLAgMBAAE=".to_string().try_into().unwrap();
    }

    fn resolver_fixture() -> Resolver {
        vec![(
            ("example.com".to_string(), "newengland".to_string()),
            (*NEWENGLAND_SELECTOR_KEY).clone(),
        )]
        .into()
    }

    mod dkim_key {
        use std::ops::Deref;

        use super::*;

        #[test]
        fn returns_key_for_a_valid_domain_selector_pair() {
            let resolver = resolver_fixture();

            let domain = "example.com";
            let selector = "newengland";

            assert_eq!(
                resolver.dkim_key(domain, selector).unwrap(),
                NEWENGLAND_SELECTOR_KEY.deref()
            )
        }

        #[test]
        fn returns_none_for_invalid_domain_selector_pairs() {
            let resolver = resolver_fixture();

            let domain = "example.com";
            let selector = "football";

            assert_eq!(resolver.dkim_key(domain, selector), None)
        }
    }
}
