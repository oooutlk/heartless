//! Provides convenient API to help generating off-by-one predicates in iterations.

use std::cell::Cell;

/// Provides a closure that evaluates to true for the first `n` times,
/// and evalutates to false otherwise.
pub fn extra( n: usize ) -> impl Fn(bool)->bool {
    let counter = Cell::new( n );
    move |cond: bool| {
        if counter.get() != 0 {
            if !cond {
                counter.set( counter.get() - 1 );
            }
            true
        } else {
            false
        }
    }
}

/// Appends `true`s to a bool expression, usually evaluating in iterations.
pub trait AndExtra: Into<bool> {
    fn and_extra( self, one: &impl Fn(bool)->bool ) -> bool {
        one( self.into() )
    }
}

impl<T> AndExtra for T where T: Into<bool> {}

#[cfg( test )]
mod tests {
    use super::*;

    #[test]
    fn test_and_extra_two() {
        let v = vec![1,2,3,4,5];
        let two = &extra(2);
        assert_eq!(
            v.iter().copied().take_while( |i| (*i<3).and_extra(two) ).collect::<Vec<_>>(),
            vec![1,2,3,4] );
    }
}
