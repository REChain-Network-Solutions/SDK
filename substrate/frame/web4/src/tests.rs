use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_runtime::traits::Hash;

#[test]
fn register_domain_should_work() {
    new_test_ext().execute_with(|| {
        // Create test data
        let domain = b"test.web4".to_vec();
        let content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
        let domain_bounded = BoundedVec::<u8, <Test as crate::Config>::MaxDomainLength>::try_from(domain).unwrap();
        let content_hash_bounded = BoundedVec::<u8, <Test as crate::Config>::MaxContentLength>::try_from(content_hash).unwrap();

        // Register domain
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            b"test.web4".to_vec(),
            b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
        ));

        // Verify domain was registered
        assert_eq!(Web4::domains(domain_bounded.clone()).unwrap().owner, 1);
        assert_eq!(Web4::domains(domain_bounded).unwrap().content_hash, content_hash_bounded);

        // Check event was emitted
        System::assert_has_event(Event::DomainRegistered {
            domain: domain_bounded,
            owner: 1,
        }.into());
    });
}

#[test]
fn register_domain_twice_should_fail() {
    new_test_ext().execute_with(|| {
        // Register domain first time
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            b"test.web4".to_vec(),
            b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
        ));

        // Try to register again - should fail
        assert_noop!(
            Web4::register_domain(
                RuntimeOrigin::signed(2),
                b"test.web4".to_vec(),
                b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
            ),
            Error::<Test>::DomainAlreadyExists
        );
    });
}

#[test]
fn update_domain_should_work() {
    new_test_ext().execute_with(|| {
        let domain = b"test.web4".to_vec();
        let initial_content = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
        let updated_content = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ_new".to_vec();

        // Register domain
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            domain.clone(),
            initial_content.clone()
        ));

        // Update domain
        assert_ok!(Web4::update_domain(
            RuntimeOrigin::signed(1),
            domain.clone(),
            updated_content.clone()
        ));

        // Verify content was updated
        let domain_bounded = BoundedVec::<u8, <Test as crate::Config>::MaxDomainLength>::try_from(domain).unwrap();
        let updated_content_bounded = BoundedVec::<u8, <Test as crate::Config>::MaxContentLength>::try_from(updated_content).unwrap();

        assert_eq!(Web4::domains(domain_bounded.clone()).unwrap().content_hash, updated_content_bounded);

        // Check event was emitted
        System::assert_has_event(Event::DomainUpdated {
            domain: domain_bounded,
            owner: 1,
        }.into());
    });
}

#[test]
fn update_domain_not_owner_should_fail() {
    new_test_ext().execute_with(|| {
        // Register domain with account 1
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            b"test.web4".to_vec(),
            b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
        ));

        // Try to update with account 2 - should fail
        assert_noop!(
            Web4::update_domain(
                RuntimeOrigin::signed(2),
                b"test.web4".to_vec(),
                b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ_new".to_vec()
            ),
            Error::<Test>::NotAuthorized
        );
    });
}

#[test]
fn transfer_domain_should_work() {
    new_test_ext().execute_with(|| {
        let domain = b"test.web4".to_vec();

        // Register domain
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            domain.clone(),
            b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
        ));

        // Transfer domain
        assert_ok!(Web4::transfer_domain(
            RuntimeOrigin::signed(1),
            domain.clone(),
            2
        ));

        // Verify ownership was transferred
        let domain_bounded = BoundedVec::<u8, <Test as crate::Config>::MaxDomainLength>::try_from(domain).unwrap();
        assert_eq!(Web4::domains(domain_bounded.clone()).unwrap().owner, 2);

        // Check event was emitted
        System::assert_has_event(Event::DomainTransferred {
            domain: domain_bounded,
            from: 1,
            to: 2,
        }.into());
    });
}

#[test]
fn transfer_domain_not_owner_should_fail() {
    new_test_ext().execute_with(|| {
        // Register domain with account 1
        assert_ok!(Web4::register_domain(
            RuntimeOrigin::signed(1),
            b"test.web4".to_vec(),
            b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec()
        ));

        // Try to transfer with account 2 - should fail
        assert_noop!(
            Web4::transfer_domain(
                RuntimeOrigin::signed(2),
                b"test.web4".to_vec(),
                3
            ),
            Error::<Test>::NotAuthorized
        );
    });
}