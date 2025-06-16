use std::sync::Arc;
use crate::types::{Exchange, TickSTKv1, TickFOPv1, BidAskSTKv1, BidAskFOPv1, QuoteSTKv1};
use crate::types::orders::OrderState;

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

/// Event handler registry that manages all callback types
pub struct EventHandlers {
    tick_callbacks: Vec<Arc<dyn TickCallback>>,
    bidask_callbacks: Vec<Arc<dyn BidAskCallback>>,
    quote_callbacks: Vec<Arc<dyn QuoteCallback>>,
    order_callbacks: Vec<Arc<dyn OrderCallback>>,
    system_callbacks: Vec<Arc<dyn SystemCallback>>,
}

impl EventHandlers {
    pub fn new() -> Self {
        Self {
            tick_callbacks: Vec::new(),
            bidask_callbacks: Vec::new(),
            quote_callbacks: Vec::new(),
            order_callbacks: Vec::new(),
            system_callbacks: Vec::new(),
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
    
    /// Trigger stock tick callbacks
    pub fn trigger_tick_stk_v1(&self, exchange: Exchange, tick: TickSTKv1) {
        for callback in &self.tick_callbacks {
            callback.on_tick_stk_v1(exchange.clone(), tick.clone());
        }
    }
    
    /// Trigger futures/options tick callbacks
    pub fn trigger_tick_fop_v1(&self, exchange: Exchange, tick: TickFOPv1) {
        for callback in &self.tick_callbacks {
            callback.on_tick_fop_v1(exchange.clone(), tick.clone());
        }
    }
    
    /// Trigger stock bid/ask callbacks
    pub fn trigger_bidask_stk_v1(&self, exchange: Exchange, bidask: BidAskSTKv1) {
        for callback in &self.bidask_callbacks {
            callback.on_bidask_stk_v1(exchange.clone(), bidask.clone());
        }
    }
    
    /// Trigger futures/options bid/ask callbacks
    pub fn trigger_bidask_fop_v1(&self, exchange: Exchange, bidask: BidAskFOPv1) {
        for callback in &self.bidask_callbacks {
            callback.on_bidask_fop_v1(exchange.clone(), bidask.clone());
        }
    }
    
    /// Trigger stock quote callbacks
    pub fn trigger_quote_stk_v1(&self, exchange: Exchange, quote: QuoteSTKv1) {
        for callback in &self.quote_callbacks {
            callback.on_quote_stk_v1(exchange.clone(), quote.clone());
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
    }
    
    /// Trigger session down callbacks
    pub fn trigger_session_down(&self) {
        for callback in &self.system_callbacks {
            callback.on_session_down();
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