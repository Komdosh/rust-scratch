#![allow(dead_code)]

mod structures;
mod primitives;
mod conditions;
mod loops;
mod scopes;
mod pm;
mod number_guessing;
mod functions;
mod traits;
mod operators;
mod dispatch;
mod memory_fundamentals;
mod multithreading;


fn main() {
    // primitives::data_types_playground();
    // primitives::constants();
    // primitives::strings();
    // primitives::enums();

    // scopes::scope_and_shadowing();

    // loops::while_loop();
    // loops::for_loop();

    // conditions::if_statement();
    // conditions::country_matcher();
    // conditions::locked_state();
    // conditions::options();

    // pm::pattern_matching();

    // number_guessing::start_game();

    // functions::functions();
    // functions::closures();
    // functions::higher_order_fn();

    // structures::stack_and_heap();
    // structures::unions();
    // structures::arrays();
    // structures::slices();
    // structures::tuples();
    // structures::generics();
    // structures::vectors();
    // structures::hash_map();
    // structures::hash_set();
    // structures::vector_objects();

    // traits::traits();
    // traits::trait_parameters();
    // traits::trait_into();
    // traits::trait_drop();

    // operators::overloading();

    // dispatch::static_dispatch();
    // dispatch::dynamic_dispatch();
    // dispatch::dispatch_in_action();

    // memory_fundamentals::ownership();
    // memory_fundamentals::borrowing();
    // memory_fundamentals::lifetime();
    // memory_fundamentals::lifetime_in_struct();
    // memory_fundamentals::ref_count_demo();
    // memory_fundamentals::atomic_ref_count_demo();
    // memory_fundamentals::circular_references();
    // memory_fundamentals::circular_references_part2();

    // multithreading::mutex();
    multithreading::threading();
}







