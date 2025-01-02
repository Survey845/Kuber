#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    fn setup() {
        // Initialize the contract (clear all wallets)
        init();
        receive_tokens("user1".to_string(), 100).unwrap();
        receive_tokens("user2".to_string(), 50).unwrap();
    }

    #[test]
    fn test_get_balance() {
        setup();

        assert_eq!(get_balance("user1".to_string()), 100);
        assert_eq!(get_balance("user2".to_string()), 50);
        assert_eq!(get_balance("non_existent".to_string()), 0);
    }

    #[test]
    fn test_send_tokens_success() {
        setup();

        let result = send_tokens("user1".to_string(), "user2".to_string(), 30);
        assert!(result.is_ok());

        assert_eq!(get_balance("user1".to_string()), 70);
        assert_eq!(get_balance("user2".to_string()), 80);
    }

    #[test]
    fn test_send_tokens_insufficient_balance() {
        setup();

        let result = send_tokens("user1".to_string(), "user2".to_string(), 200);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient balance");

        assert_eq!(get_balance("user1".to_string()), 100);
        assert_eq!(get_balance("user2".to_string()), 50);
    }

    #[test]
    fn test_send_tokens_sender_not_found() {
        setup();

        let result = send_tokens("non_existent".to_string(), "user2".to_string(), 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender wallet not found");

        assert_eq!(get_balance("user2".to_string()), 50);
    }

    #[test]
    fn test_receive_tokens() {
        setup();

        let result = receive_tokens("user1".to_string(), 50);
        assert!(result.is_ok());
        assert_eq!(get_balance("user1".to_string()), 150);

        let result_new_wallet = receive_tokens("new_user".to_string(), 25);
        assert!(result_new_wallet.is_ok());
        assert_eq!(get_balance("new_user".to_string()), 25);
    }
}
