//! Opt-in evidence that every byte of one frame reached the sending socket.

use std::ops::Range;

/// One application's token and the stream bytes not yet accepted by the UDP socket.
#[derive(Debug)]
pub(crate) struct Receipt {
    pub(crate) token: u64,
    left: Vec<Range<u64>>,
}

impl Receipt {
    pub(crate) fn new(token: u64, bytes: Range<u64>) -> Self {
        Self { token, left: Vec::from([bytes]) }
    }

    /// A later range cannot excuse an earlier hole; a retransmission is not fresh progress.
    pub(crate) fn sent(&mut self, bytes: &Range<u64>) -> bool {
        let mut left = Vec::new();
        for range in self.left.drain(..) {
            if range.end <= bytes.start || range.start >= bytes.end {
                left.push(range);
                continue;
            }
            if range.start < bytes.start {
                left.push(range.start..bytes.start);
            }
            if range.end > bytes.end {
                left.push(bytes.end..range.end);
            }
        }
        self.left = left;
        self.left.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_lab_receipt_needs_every_frame_byte_sent_not_queued() {
        let mut receipt = Receipt::new(7, 8..108);
        assert!(!receipt.sent(&(58..108)));
        assert!(!receipt.sent(&(58..108)));
        assert!(!receipt.sent(&(0..28)));
        assert!(!receipt.sent(&(38..58)));
        assert!(receipt.sent(&(28..38)));
        assert_eq!(receipt.token, 7);
    }
}
