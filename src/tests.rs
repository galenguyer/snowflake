use super::*;

#[test]
#[should_panic]
fn test_machine_id_limit() {
    SnowflakeGenerator::new(32, 0);
}

#[test]
#[should_panic]
fn test_thread_id_limit() {
    SnowflakeGenerator::new(0, 32);
}

#[test]
fn test_unique() {
    let mut generator = SnowflakeGenerator::new(0, 0);
    assert_ne!(generator.generate(), generator.generate());
}

#[test]
fn test_unique_generator() {
    let mut generator1 = SnowflakeGenerator::new(0, 0);
    let mut generator2 = SnowflakeGenerator::new(0, 1);
    assert_ne!(generator1.generate(), generator2.generate());
}

#[test]
fn test_identical_generator() {
    let mut generator1 = SnowflakeGenerator::new(0, 0);
    let mut generator2 = SnowflakeGenerator::new(0, 0);
    assert_eq!(generator1.generate(), generator2.generate());
}

#[test]
fn test_many_unique() {
    let mut generator = SnowflakeGenerator::new(0, 0);
    let mut ids = Vec::new();
    for _ in 0..10_000 {
        ids.push(generator.generate());
    }
    ids.sort();
    for i in 0..ids.len() - 1 {
        assert_ne!(ids[i], ids[i + 1]);
    }
}

#[test]
fn test_conversion() {
    let mut generator = SnowflakeGenerator::new(0, 0);
    let id = generator.generate();
    let snowflake = Snowflake::from(id);
    assert_eq!(id, u64::from(snowflake));
}
