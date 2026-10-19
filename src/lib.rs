use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EngineError {
    #[error("Buffer capacity exceeded: requested {requested}, available {available}")]
    BufferOverflow { requested: usize, available: usize },
    #[error("Validation failed: {0}")]
    ValidationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamEvent {
    pub id: String,
    pub payload: String,
    pub timestamp_ms: u64,
}

pub struct RingBufferStream {
    capacity: usize,
    buffer: Vec<StreamEvent>,
}

impl RingBufferStream {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be strictly positive");
        Self {
            capacity,
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn push(&mut self, event: StreamEvent) -> Result<(), EngineError> {
        if event.id.trim().is_empty() {
            return Err(EngineError::ValidationError("Event ID cannot be empty".into()));
        }
        if self.buffer.len() >= self.capacity {
            return Err(EngineError::BufferOverflow {
                requested: 1,
                available: 0,
            });
        }
        self.buffer.push(event);
        Ok(())
    }

    pub fn flush_batch(&mut self, batch_size: usize) -> Vec<StreamEvent> {
        let drain_count = batch_size.min(self.buffer.len());
        self.buffer.drain(0..drain_count).collect()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_drain() {
        let mut stream = RingBufferStream::new(2);
        let e1 = StreamEvent { id: "1".into(), payload: "a".into(), timestamp_ms: 10 };
        let e2 = StreamEvent { id: "2".into(), payload: "b".into(), timestamp_ms: 20 };
        assert!(stream.push(e1.clone()).is_ok());
        assert!(stream.push(e2.clone()).is_ok());

        assert_eq!(stream.len(), 2);
        let drained = stream.flush_batch(1);
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0], e1);
        assert_eq!(stream.len(), 1);
    }

    #[test]
    fn test_overflow() {
        let mut stream = RingBufferStream::new(1);
        let e1 = StreamEvent { id: "1".into(), payload: "a".into(), timestamp_ms: 10 };
        let e2 = StreamEvent { id: "2".into(), payload: "b".into(), timestamp_ms: 20 };
        assert!(stream.push(e1).is_ok());
        assert_eq!(stream.push(e2), Err(EngineError::BufferOverflow { requested: 1, available: 0 }));
    }
}
