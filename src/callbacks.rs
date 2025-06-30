use crate::types::orders::OrderState;
use crate::types::{
    BidAskFOPv1, BidAskSTKv1, Exchange, QuoteSTKv1, SecurityType, TickFOPv1, TickSTKv1,
};
use std::sync::Arc;

/// Type alias for event closure to reduce complexity
type EventClosure = Arc<dyn Fn(i32, i32, String, String) + Send + Sync>;
/// Type alias for session down closure
type SessionDownClosure = Arc<dyn Fn() + Send + Sync>;

/// Trait for handling market data tick events
pub trait TickCallback: Send + Sync {
    /// Called when a stock tick event occurs
    fn on_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1);

    /// Called when a futures/options tick event occurs
    fn on_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1);
}

/// Trait for handling bid/ask spread events
pub trait BidAskCallback: Send + Sync {
    /// Called when a stock bid/ask event occurs
    fn on_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1);

    /// Called when a futures/options bid/ask event occurs
    fn on_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1);
}

/// Trait for handling quote events
pub trait QuoteCallback: Send + Sync {
    /// Called when a stock quote event occurs
    fn on_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1);

    /// Called when a general quote event occurs
    fn on_quote(&self, topic: String, data: serde_json::Value);
}

/// Trait for handling order events
pub trait OrderCallback: Send + Sync {
    /// Called when an order status changes
    fn on_order(&self, order_state: OrderState, data: serde_json::Value);
}

/// Trait for handling system events
pub trait SystemCallback: Send + Sync {
    /// Called when system events occur
    fn on_event(&self, event_type: i32, code: i32, message: String, details: String);

    /// Called when session is disconnected
    fn on_session_down(&self);
}

/// Trait for handling contract fetch events (對應原始 Python 的 contracts_cb)
///
/// 對應原始 Python 函數簽名：
/// ```python
/// contracts_cb: typing.Callable[[SecurityType], None] = None
/// ```
pub trait ContractCallback: Send + Sync {
    /// Called when contracts of a specific security type are fetched
    /// 對應原始 Python: contracts_cb(securitytype)
    fn on_contracts_fetched(&self, security_type: SecurityType);

    /// Called when all contracts fetch is completed
    fn on_all_contracts_fetched(&self);
}

/// Event handler registry that manages all callback types
pub struct EventHandlers {
    tick_callbacks: Vec<Arc<dyn TickCallback>>,
    bidask_callbacks: Vec<Arc<dyn BidAskCallback>>,
    quote_callbacks: Vec<Arc<dyn QuoteCallback>>,
    order_callbacks: Vec<Arc<dyn OrderCallback>>,
    system_callbacks: Vec<Arc<dyn SystemCallback>>,
    contract_callbacks: Vec<Arc<dyn ContractCallback>>,
    // Direct function closures for flexibility
    event_closures: Vec<EventClosure>,
    session_down_closures: Vec<SessionDownClosure>,
}

impl EventHandlers {
    pub fn new() -> Self {
        Self {
            tick_callbacks: Vec::new(),
            bidask_callbacks: Vec::new(),
            quote_callbacks: Vec::new(),
            order_callbacks: Vec::new(),
            system_callbacks: Vec::new(),
            contract_callbacks: Vec::new(),
            event_closures: Vec::new(),
            session_down_closures: Vec::new(),
        }
    }

    /// Register a tick data callback handler
    pub fn register_tick_callback(&mut self, callback: Arc<dyn TickCallback>) {
        self.tick_callbacks.push(callback);
    }

    /// Register a bid/ask callback handler
    pub fn register_bidask_callback(&mut self, callback: Arc<dyn BidAskCallback>) {
        self.bidask_callbacks.push(callback);
    }

    /// Register a quote callback handler
    pub fn register_quote_callback(&mut self, callback: Arc<dyn QuoteCallback>) {
        self.quote_callbacks.push(callback);
    }

    /// Register an order callback handler
    pub fn register_order_callback(&mut self, callback: Arc<dyn OrderCallback>) {
        self.order_callbacks.push(callback);
    }

    /// Register a system callback handler
    pub fn register_system_callback(&mut self, callback: Arc<dyn SystemCallback>) {
        self.system_callbacks.push(callback);
    }

    /// Register a contract callback handler
    pub fn register_contract_callback(&mut self, callback: Arc<dyn ContractCallback>) {
        self.contract_callbacks.push(callback);
    }

    /// Register an event closure (for direct function callbacks)
    pub fn register_event_closure(
        &mut self,
        callback: Arc<dyn Fn(i32, i32, String, String) + Send + Sync>,
    ) {
        self.event_closures.push(callback);
    }

    /// Register a session down closure (for direct function callbacks)
    pub fn register_session_down_closure(&mut self, callback: Arc<dyn Fn() + Send + Sync>) {
        self.session_down_closures.push(callback);
    }

    /// Trigger stock tick callbacks
    pub fn trigger_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        for callback in &self.tick_callbacks {
            callback.on_tick_stk_v1(exchange, tick.clone());
        }
    }

    /// Trigger futures/options tick callbacks
    pub fn trigger_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        for callback in &self.tick_callbacks {
            callback.on_tick_fop_v1(exchange, tick.clone());
        }
    }

    /// Trigger stock bid/ask callbacks
    pub fn trigger_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        for callback in &self.bidask_callbacks {
            callback.on_bidask_stk_v1(exchange, bidask.clone());
        }
    }

    /// Trigger futures/options bid/ask callbacks
    pub fn trigger_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        for callback in &self.bidask_callbacks {
            callback.on_bidask_fop_v1(exchange, bidask.clone());
        }
    }

    /// Trigger stock quote callbacks
    pub fn trigger_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1) {
        for callback in &self.quote_callbacks {
            callback.on_quote_stk_v1(exchange, quote.clone());
        }
    }

    /// Trigger general quote callbacks
    pub fn trigger_quote(&self, topic: String, data: serde_json::Value) {
        for callback in &self.quote_callbacks {
            callback.on_quote(topic.clone(), data.clone());
        }
    }

    /// Trigger order callbacks
    pub fn trigger_order(&self, order_state: OrderState, data: serde_json::Value) {
        for callback in &self.order_callbacks {
            callback.on_order(order_state.clone(), data.clone());
        }
    }

    /// Trigger system event callbacks
    pub fn trigger_event(&self, event_type: i32, code: i32, message: String, details: String) {
        for callback in &self.system_callbacks {
            callback.on_event(event_type, code, message.clone(), details.clone());
        }
        for closure in &self.event_closures {
            closure(event_type, code, message.clone(), details.clone());
        }
    }

    /// Trigger session down callbacks
    pub fn trigger_session_down(&self) {
        for callback in &self.system_callbacks {
            callback.on_session_down();
        }
        for closure in &self.session_down_closures {
            closure();
        }
    }

    /// Trigger contract fetched callbacks for specific security type
    pub fn trigger_contracts_fetched(&self, security_type: SecurityType) {
        for callback in &self.contract_callbacks {
            callback.on_contracts_fetched(security_type.clone());
        }
    }

    /// Trigger all contracts fetched callbacks
    pub fn trigger_all_contracts_fetched(&self) {
        for callback in &self.contract_callbacks {
            callback.on_all_contracts_fetched();
        }
    }
}

impl Default for EventHandlers {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for implementing multiple callback traits on a single struct
#[macro_export]
macro_rules! impl_callbacks {
    ($struct:ident, $($trait:ident),+) => {
        $(
            impl $trait for $struct {}
        )+
    };
}
