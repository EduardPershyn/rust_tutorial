//! Exercise 02: enums, with and without data; a state machine.
//! Run: cargo test --lib ex02
//! Rule for this file: no `_` wildcard arms. List the variants, so adding one forces you to update the code.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    /// The cycle is Red → Green → Yellow → Red.
    pub fn next(self) -> Light {
        todo!()
    }

    /// Red lasts 30 s, Green 25 s, Yellow 5 s.
    pub fn duration_secs(self) -> u32 {
        todo!()
    }
}

/// An order's life cycle. Each state carries only the data that makes sense in that state.
///
/// Allowed transitions:
///   Created --pay--> Paid --ship--> Shipped --deliver--> Delivered
///   Created --cancel--> Cancelled { refund: 0 }
///   Paid    --cancel--> Cancelled { refund: amount }
///
/// Every transition method returns true on success. On a transition that isn't allowed,
/// it returns false and leaves the order unchanged.
#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Created,
    Paid { amount: u64 },
    Shipped { amount: u64, tracking: String },
    Delivered,
    Cancelled { refund: u64 },
}

impl Order {
    /// Created → Paid.
    /// Hint: to change the state, assign a new value through the reference: `*self = Order::…;`
    pub fn pay(&mut self, amount: u64) -> bool {
        todo!()
    }

    /// Paid → Shipped (keeps the amount).
    pub fn ship(&mut self, tracking: &str) -> bool {
        todo!()
    }

    /// Shipped → Delivered.
    pub fn deliver(&mut self) -> bool {
        todo!()
    }

    /// Created → Cancelled { refund: 0 }, Paid → Cancelled { refund: amount }.
    pub fn cancel(&mut self) -> bool {
        todo!()
    }

    /// The tracking number while Shipped, otherwise None. Borrowed, no allocation.
    pub fn tracking(&self) -> Option<&str> {
        todo!()
    }

    /// Human-readable status:
    /// "created", "paid 100", "shipped via TRK-1", "delivered", "cancelled, refund 100"
    pub fn status(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_cycle() {
        assert_eq!(Light::Red.next(), Light::Green);
        assert_eq!(Light::Green.next(), Light::Yellow);
        assert_eq!(Light::Yellow.next(), Light::Red);
    }

    #[test]
    fn light_durations() {
        let mut light = Light::Red;
        let mut total = 0;
        for _ in 0..3 {
            total += light.duration_secs();
            light = light.next();
        }
        assert_eq!(total, 60);
        assert_eq!(Light::Yellow.duration_secs(), 5);
    }

    #[test]
    fn order_happy_path() {
        let mut o = Order::Created;
        assert!(o.pay(100));
        assert_eq!(o, Order::Paid { amount: 100 });
        assert!(o.ship("TRK-1"));
        assert_eq!(o, Order::Shipped { amount: 100, tracking: "TRK-1".to_string() });
        assert!(o.deliver());
        assert_eq!(o, Order::Delivered);
    }

    #[test]
    fn invalid_transitions_change_nothing() {
        let mut o = Order::Created;
        assert!(!o.ship("X"));
        assert!(!o.deliver());
        assert_eq!(o, Order::Created);

        assert!(o.pay(50));
        assert!(!o.pay(60));
        assert!(!o.deliver());
        assert_eq!(o, Order::Paid { amount: 50 });

        let mut done = Order::Delivered;
        assert!(!done.pay(1));
        assert!(!done.ship("Y"));
        assert!(!done.deliver());
        assert!(!done.cancel());
        assert_eq!(done, Order::Delivered);
    }

    #[test]
    fn cancel_refunds() {
        let mut a = Order::Created;
        assert!(a.cancel());
        assert_eq!(a, Order::Cancelled { refund: 0 });

        let mut b = Order::Paid { amount: 70 };
        assert!(b.cancel());
        assert_eq!(b, Order::Cancelled { refund: 70 });

        let mut c = Order::Shipped { amount: 70, tracking: "T".to_string() };
        assert!(!c.cancel());
        assert_eq!(c, Order::Shipped { amount: 70, tracking: "T".to_string() });

        assert!(!a.cancel()); // already cancelled
    }

    #[test]
    fn tracking_only_when_shipped() {
        let shipped = Order::Shipped { amount: 1, tracking: "TRK-9".to_string() };
        assert_eq!(shipped.tracking(), Some("TRK-9"));
        assert_eq!(Order::Created.tracking(), None);
        assert_eq!(Order::Paid { amount: 1 }.tracking(), None);
        assert_eq!(Order::Delivered.tracking(), None);
    }

    #[test]
    fn status_text() {
        assert_eq!(Order::Created.status(), "created");
        assert_eq!(Order::Paid { amount: 100 }.status(), "paid 100");
        let shipped = Order::Shipped { amount: 100, tracking: "TRK-1".to_string() };
        assert_eq!(shipped.status(), "shipped via TRK-1");
        assert_eq!(Order::Delivered.status(), "delivered");
        assert_eq!(Order::Cancelled { refund: 100 }.status(), "cancelled, refund 100");
    }
}
