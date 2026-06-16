use obscura_cdp::dispatch::CdpContext;

#[test]
fn default_context_ids_are_registered() {
    let ctx = CdpContext::new();

    assert!(ctx.valid_context_ids.contains(&1));
    assert!(ctx.valid_context_ids.contains(&2));
}

#[test]
fn isolated_world_context_ids_are_monotonic_and_registered() {
    let mut ctx = CdpContext::new();

    let first = ctx.next_isolated_context();
    let second = ctx.next_isolated_context();

    assert_eq!(first, 100);
    assert_eq!(second, 101);
    assert!(ctx.valid_context_ids.contains(&first));
    assert!(ctx.valid_context_ids.contains(&second));
    assert!(second > first);
}
