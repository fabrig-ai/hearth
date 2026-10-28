use hearth_git_world::{
    fixtures::{self, SMALL},
    world,
};
use hearth_protocol_git::limits;
use skein_heap::{Counting, Meter};
#[global_allocator]
static HEAP: Counting = Counting;
#[test]
fn each_decoder_at_its_limits_holds_no_more_than_its_worst_case() {
    for case in fixtures::at_the_limits(&SMALL) {
        let bound = limits::worst_case(case.decoder, &SMALL).expect("profile priced");
        let mut run = world::Stepped::new(&case, &SMALL);
        let meter = Meter::new();
        meter.start();
        run.open();
        let step = meter.end();
        run.drop_events();
        // Growth holds old and new buffers at the peak; after output handoff,
        // held decoder storage stays within the formula, peak within twice it.
        meter.check(step, 2 * bound, &case.what);
        assert!(meter.held() <= bound, "{}: held decoder storage within its formula", case.what);
        while run.pending() {
            meter.start();
            run.call();
            let step = meter.end();
            run.drop_events();
            meter.check(step, 2 * bound, &case.what);
            assert!(meter.held() <= bound, "{}: held decoder storage within its formula", case.what);
        }
    }
}
