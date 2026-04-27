//! Async Channel service for business logic related to channels
//!
//! This async service handles channel-related operations including validation,
//! filtering, and channel information management.

use std::collections::HashMap;
use crate::screen::channel_dialog::ChannelInfo;

/// Async service for channel-related business logic
///
/// The `AsyncChannelService` provides validation, lookup, and filtering
/// capabilities for HKGolden channels.
pub struct AsyncChannelService {
    channels: Vec<ChannelInfo>,
    channel_map: HashMap<String, usize>,
}

impl AsyncChannelService {
    /// Create a new AsyncChannelService with default channels
    ///
    /// # Returns
    /// A new channel service with all 42 HKGolden channels loaded
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncChannelService;
    ///
    /// let service = AsyncChannelService::new();
    /// assert_eq!(service.channel_count(), 42);
    /// ```
    pub fn new() -> Self {
        let channels = Self::default_channels();
        let channel_map = Self::build_channel_map(&channels);

        Self {
            channels,
            channel_map,
        }
    }

    /// Get the total number of channels
    ///
    /// # Returns
    /// Number of channels
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    /// Check if a channel code is valid
    ///
    /// # Arguments
    /// * `channel` - Channel code to validate
    ///
    /// # Returns
    /// true if the channel code exists, false otherwise
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncChannelService;
    ///
    /// let service = AsyncChannelService::new();
    /// assert!(service.is_valid_channel("BW"));
    /// assert!(!service.is_valid_channel("INVALID"));
    /// ```
    pub fn is_valid_channel(&self, channel: &str) -> bool {
        self.channel_map.contains_key(channel)
    }

    /// Get channel information by index
    ///
    /// # Arguments
    /// * `index` - Channel index (0-based)
    ///
    /// # Returns
    /// Option containing channel information if index is valid
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncChannelService;
    ///
    /// let service = AsyncChannelService::new();
    /// let channel = service.get_channel_by_index(0);
    /// assert!(channel.is_some());
    /// assert_eq!(channel.unwrap().code, "BW");
    /// ```
    pub fn get_channel_by_index(&self, index: usize) -> Option<&ChannelInfo> {
        self.channels.get(index)
    }

    /// Get channel information by code
    ///
    /// # Arguments
    /// * `channel_code` - Channel code
    ///
    /// # Returns
    /// Option containing channel information if code exists
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncChannelService;
    ///
    /// let service = AsyncChannelService::new();
    /// let channel = service.get_channel("BW");
    /// assert!(channel.is_some());
    /// assert_eq!(channel.unwrap().title, "吹水台");
    /// ```
    pub fn get_channel(&self, channel_code: &str) -> Option<&ChannelInfo> {
        self.channel_map
            .get(channel_code)
            .and_then(|&index| self.channels.get(index))
    }

    /// Filter channels by pattern (case-insensitive)
    ///
    /// # Arguments
    /// * `pattern` - Filter pattern for channel code or title
    ///
    /// # Returns
    /// Vector of indices of matching channels
    ///
    /// # Examples
    /// ```
    /// use hkg::async_services::AsyncChannelService;
    ///
    /// let service = AsyncChannelService::new();
    /// let results = service.filter_channels("台");
    /// assert!(!results.is_empty());
    /// ```
    pub fn filter_channels(&self, pattern: &str) -> Vec<usize> {
        let pattern_lower = pattern.to_lowercase();

        self.channels
            .iter()
            .enumerate()
            .filter(|(_, channel)| {
                channel.channel.to_lowercase().contains(&pattern_lower)
                    || channel.title.to_lowercase().contains(&pattern_lower)
            })
            .map(|(index, _)| index)
            .collect()
    }

    /// Build a HashMap for quick channel lookup by code
    fn build_channel_map(channels: &[ChannelInfo]) -> HashMap<String, usize> {
        channels
            .iter()
            .enumerate()
            .map(|(index, channel)| (channel.channel.clone(), index))
            .collect()
    }

    /// Define the default set of 42 HKGolden channels
    fn default_channels() -> Vec<ChannelInfo> {
        vec![
            ChannelInfo { title: "吹水台".to_string(), channel: "BW".to_string() },
            ChannelInfo { title: "硬件".to_string(), channel: "HW".to_string() },
            ChannelInfo { title: "Newton".to_string(), channel: "NW".to_string() },
            ChannelInfo { title: "軟件".to_string(), channel: "SW".to_string() },
            ChannelInfo { title: "電訊".to_string(), channel: "TN".to_string() },
            ChannelInfo { title: "網事".to_string(), channel: "NW".to_string() },
            ChannelInfo { title: "Apps".to_string(), channel: "AP".to_string() },
            ChannelInfo { title: "遊戲".to_string(), channel: "GM".to_string() },
            ChannelInfo { title: "主機".to_string(), channel: "CN".to_string() },
            ChannelInfo { title: "電視".to_string(), channel: "TV".to_string() },
            ChannelInfo { title: "影視".to_string(), channel: "MV".to_string() },
            ChannelInfo { title: "音樂".to_string(), channel: "MU".to_string() },
            ChannelInfo { title: "潮流".to_string(), channel: "TR".to_string() },
            ChannelInfo { title: "旅遊".to_string(), channel: "TV".to_string() },
            ChannelInfo { title: "美食".to_string(), channel: "FD".to_string() },
            ChannelInfo { title: "財經".to_string(), channel: "FN".to_string() },
            ChannelInfo { title: "投資".to_string(), channel: "IN".to_string() },
            ChannelInfo { title: "體育".to_string(), channel: "SP".to_string() },
            ChannelInfo { title: "動畫".to_string(), channel: "AN".to_string() },
            ChannelInfo { title: "漫畫".to_string(), channel: "CO".to_string() },
            ChannelInfo { title: "玩物".to_string(), channel: "TO".to_string() },
            ChannelInfo { title: "講呢啲".to_string(), channel: "TS".to_string() },
            ChannelInfo { title: "感情".to_string(), channel: "LO".to_string() },
            ChannelInfo { title: "成人".to_string(), channel: "AD".to_string() },
            ChannelInfo { title: "時事".to_string(), channel: "CU".to_string() },
            ChannelInfo { title: "政改".to_string(), channel: "PO".to_string() },
            ChannelInfo { title: "社區".to_string(), channel: "SC".to_string() },
            ChannelInfo { title: "學術".to_string(), channel: "AC".to_string() },
            ChannelInfo { title: "職場".to_string(), channel: "WK".to_string() },
            ChannelInfo { title: "校園".to_string(), channel: "SC".to_string() },
            ChannelInfo { title: "汽車".to_string(), channel: "AU".to_string() },
            ChannelInfo { title: "電腦".to_string(), channel: "PC".to_string() },
            ChannelInfo { title: "電子".to_string(), channel: "EL".to_string() },
            ChannelInfo { title: "數碼".to_string(), channel: "DG".to_string() },
            ChannelInfo { title: "攝影".to_string(), channel: "PH".to_string() },
            ChannelInfo { title: "AV".to_string(), channel: "AV".to_string() },
            ChannelInfo { title: "手表".to_string(), channel: "WA".to_string() },
            ChannelInfo { title: "健康".to_string(), channel: "HE".to_string() },
            ChannelInfo { title: "天地".to_string(), channel: "NC".to_string() },
            ChannelInfo { title: "寵物".to_string(), channel: "PT".to_string() },
            ChannelInfo { title: "文學".to_string(), channel: "LT".to_string() },
            ChannelInfo { title: "自娛".to_string(), channel: "EN".to_string() },
        ]
    }
}

impl Default for AsyncChannelService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let service = AsyncChannelService::new();
        assert_eq!(service.channel_count(), 42);
    }

    #[test]
    fn test_is_valid_channel() {
        let service = AsyncChannelService::new();
        assert!(service.is_valid_channel("BW"));
        assert!(service.is_valid_channel("HW"));
        assert!(!service.is_valid_channel("INVALID"));
        assert!(!service.is_valid_channel(""));
    }

    #[test]
    fn test_get_channel_by_index() {
        let service = AsyncChannelService::new();
        let channel = service.get_channel_by_index(0);
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().channel, "BW");

        let invalid = service.get_channel_by_index(100);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_get_channel() {
        let service = AsyncChannelService::new();
        let channel = service.get_channel("BW");
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().title, "吹水台");

        let invalid = service.get_channel("INVALID");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_filter_channels() {
        let service = AsyncChannelService::new();
        let results = service.filter_channels("台");
        assert!(!results.is_empty());

        let results = service.filter_channels("硬");
        assert!(!results.is_empty());

        let results = service.filter_channels("INVALID");
        assert!(results.is_empty());
    }
}
