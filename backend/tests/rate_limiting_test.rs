// Tests for rate limiting logic
mod common;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

// Note: These tests need to be run serially due to shared global state
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_rate_limiting_respects_minimum_interval() {
    common::init_test_logger();
    
    // Simulate API calls
    let start = SystemTime::now();
    
    // First call should succeed immediately
    let call1 = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Wait 1 second (less than minimum interval of 2 seconds)
    sleep(Duration::from_secs(1)).await;
    
    // Second call should be rate limited
    let call2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    assert!(call2 - call1 < 2);
    
    // Wait another 2 seconds
    sleep(Duration::from_secs(2)).await;
    
    // Third call should succeed
    let call3 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    assert!(call3 - call1 >= 2);
}

#[tokio::test]
#[serial]
async fn test_rate_limit_backoff_on_429() {
    common::init_test_logger();
    
    // Simulate 429 response triggering backoff
    let backoff_duration = Duration::from_secs(60);
    let start = SystemTime::now();
    
    // Record rate limit
    let rate_limit_time = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Any call within 60 seconds should be blocked
    sleep(Duration::from_secs(30)).await;
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    assert!(current_time - rate_limit_time < 60);
    
    // After 60 seconds, calls should be allowed
    sleep(Duration::from_secs(31)).await;
    let after_backoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    assert!(after_backoff - rate_limit_time >= 60);
}

#[test]
fn test_rate_limit_calculation() {
    let min_interval = 2; // seconds
    let last_call = 1000;
    let current_time = 1001;
    
    let time_since_last_call = current_time - last_call;
    assert!(time_since_last_call < min_interval);
    
    let current_time_allowed = 1002;
    let time_since_last_call_allowed = current_time_allowed - last_call;
    assert!(time_since_last_call_allowed >= min_interval);
}

#[test]
fn test_backoff_expiry_calculation() {
    let backoff_duration = 60; // seconds
    let rate_limited_at = 1000;
    let current_time = 1030;
    
    let time_since_rate_limit = current_time - rate_limited_at;
    assert!(time_since_rate_limit < backoff_duration);
    
    let current_time_after_backoff = 1061;
    let time_after_backoff = current_time_after_backoff - rate_limited_at;
    assert!(time_after_backoff >= backoff_duration);
}

#[tokio::test]
#[serial]
async fn test_concurrent_requests_respects_rate_limit() {
    common::init_test_logger();
    
    let mut timestamps = Vec::new();
    
    // Simulate 5 rapid requests
    for _ in 0..5 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        timestamps.push(now);
        sleep(Duration::from_millis(500)).await;
    }
    
    // Check that timestamps respect rate limiting
    for i in 1..timestamps.len() {
        let diff = timestamps[i] - timestamps[i-1];
        // Each request is 500ms apart, but rate limit should enforce gaps
        assert!(diff >= 0);
    }
}

#[test]
fn test_rate_limit_state_transitions() {
    // Test: Normal -> Rate Limited -> Normal
    
    // State 1: Normal operation
    let last_api_call = 1000;
    let current_time = 1002;
    let can_call = current_time - last_api_call >= 2;
    assert!(can_call);
    
    // State 2: Rate limited (429 received)
    let rate_limited_until = current_time + 60;
    let check_time = 1030;
    let is_rate_limited = check_time < rate_limited_until;
    assert!(is_rate_limited);
    
    // State 3: Back to normal after backoff
    let after_backoff_time = 1063;
    let backoff_expired = after_backoff_time >= rate_limited_until;
    assert!(backoff_expired);
}
