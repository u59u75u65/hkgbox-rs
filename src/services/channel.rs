//! Channel service for business logic related to channels
//!
//! This service handles channel-related operations including validation,
//! filtering, and channel information management.

use crate::screen::channel_dialog::ChannelInfo;
use std::collections::HashMap;

/// Service for channel-related business logic
///
/// The `ChannelService` encapsulates all business logic related to channels,
/// providing validation, lookup, and filtering capabilities.
pub struct ChannelService {
    channels: Vec<ChannelInfo>,
    channel_map: HashMap<String, usize>,
}

impl ChannelService {
    /// Create a new channel service with default channels
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ChannelService;
    ///
    /// let service = ChannelService::new();
    /// assert!(service.get_all_channels().len() > 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let channels = Self::default_channels();
        let channel_map = Self::build_channel_map(&channels);
        Self { channels, channel_map }
    }

    /// Create a new channel service with custom channels
    ///
    /// # Arguments
    /// * `channels` - Custom channel list
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ChannelService;
    /// use hkg::screen::channel_dialog::ChannelInfo;
    ///
    /// let channels = vec![
    ///     ChannelInfo {
    ///         title: "Test".to_string(),
    ///         channel: "TS".to_string(),
    ///     },
    /// ];
    /// let service = ChannelService::with_channels(channels);
    /// ```
    #[must_use]
    pub fn with_channels(channels: Vec<ChannelInfo>) -> Self {
        let channel_map = Self::build_channel_map(&channels);
        Self { channels, channel_map }
    }

    /// Get all available channels
    ///
    /// # Returns
    /// Slice of all channel information
    #[must_use]
    pub fn get_all_channels(&self) -> &[ChannelInfo] {
        &self.channels
    }

    /// Get channel by code
    ///
    /// # Arguments
    /// * `code` - Channel code (e.g., "BW", "HT")
    ///
    /// # Returns
    /// `Some(ChannelInfo)` if found, `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ChannelService;
    ///
    /// let service = ChannelService::new();
    /// let channel = service.get_channel("BW");
    /// assert!(channel.is_some());
    /// assert_eq!(channel.unwrap().title, "吹水台");
    /// ```
    #[must_use]
    pub fn get_channel(&self, code: &str) -> Option<&ChannelInfo> {
        self.channel_map.get(code)
            .and_then(|&index| self.channels.get(index))
    }

    /// Get channel by index
    ///
    /// # Arguments
    /// * `index` - Channel index (1-based)
    ///
    /// # Returns
    /// `Some(ChannelInfo)` if found, `None` otherwise
    #[must_use]
    pub fn get_channel_by_index(&self, index: usize) -> Option<&ChannelInfo> {
        if index > 0 && index <= self.channels.len() {
            self.channels.get(index - 1)
        } else {
            None
        }
    }

    /// Validate channel code
    ///
    /// # Arguments
    /// * `code` - Channel code to validate
    ///
    /// # Returns
    /// `true` if channel exists, `false` otherwise
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ChannelService;
    ///
    /// let service = ChannelService::new();
    /// assert!(service.is_valid_channel("BW"));
    /// assert!(!service.is_valid_channel("INVALID"));
    /// ```
    #[must_use]
    pub fn is_valid_channel(&self, code: &str) -> bool {
        self.channel_map.contains_key(code)
    }

    /// Filter channels by pattern
    ///
    /// # Arguments
    /// * `pattern` - Search pattern (case-insensitive)
    ///
    /// # Returns
    /// Vector of matching channel indices (1-based)
    ///
    /// # Examples
    /// ```
    /// use hkg::services::ChannelService;
    ///
    /// let service = ChannelService::new();
    /// let results = service.filter_channels("台");
    /// assert!(!results.is_empty());
    /// ```
    #[must_use]
    pub fn filter_channels(&self, pattern: &str) -> Vec<usize> {
        if pattern.is_empty() {
            return Vec::new();
        }

        let pattern_lower = pattern.to_lowercase();
        self.channels.iter()
            .enumerate()
            .filter(|(_, channel)| {
                let title_lower = channel.title.to_lowercase();
                let channel_lower = channel.channel.to_lowercase();
                title_lower.contains(&pattern_lower) || channel_lower.contains(&pattern_lower)
            })
            .map(|(i, _)| i + 1) // Convert to 1-based index
            .collect()
    }

    /// Get default channels
    fn default_channels() -> Vec<ChannelInfo> {
        vec![
            // 推薦
            ChannelInfo { title: "吹水台".to_string(), channel: "BW".to_string() },
            ChannelInfo { title: "高登熱".to_string(), channel: "HT".to_string() },
            ChannelInfo { title: "最　新".to_string(), channel: "NW".to_string() },
            ChannelInfo { title: "時事台".to_string(), channel: "CA".to_string() },
            ChannelInfo { title: "娛樂台".to_string(), channel: "ET".to_string() },
            ChannelInfo { title: "體育台".to_string(), channel: "SP".to_string() },
            ChannelInfo { title: "財經台".to_string(), channel: "FN".to_string() },
            ChannelInfo { title: "學術台".to_string(), channel: "ST".to_string() },
            ChannelInfo { title: "講故台".to_string(), channel: "SY".to_string() },
            ChannelInfo { title: "創意台".to_string(), channel: "EP".to_string() },
            ChannelInfo { title: "超自然台".to_string(), channel: "SN".to_string() },
            ChannelInfo { title: "優惠台".to_string(), channel: "CP".to_string() },
            // 科技
            ChannelInfo { title: "硬件台".to_string(), channel: "HW".to_string() },
            ChannelInfo { title: "電訊台".to_string(), channel: "IN".to_string() },
            ChannelInfo { title: "軟件台".to_string(), channel: "SW".to_string() },
            ChannelInfo { title: "手機台".to_string(), channel: "MP".to_string() },
            ChannelInfo { title: "Apps台".to_string(), channel: "AP".to_string() },
            ChannelInfo { title: "Crypto台".to_string(), channel: "blockchain".to_string() },
            ChannelInfo { title: "AI技術台".to_string(), channel: "AI".to_string() },
            // 消閒
            ChannelInfo { title: "遊戲台".to_string(), channel: "GM".to_string() },
            ChannelInfo { title: "飲食台".to_string(), channel: "ED".to_string() },
            ChannelInfo { title: "旅遊台".to_string(), channel: "TR".to_string() },
            ChannelInfo { title: "潮流台".to_string(), channel: "CO".to_string() },
            ChannelInfo { title: "動漫台".to_string(), channel: "AN".to_string() },
            ChannelInfo { title: "玩具台".to_string(), channel: "TO".to_string() },
            ChannelInfo { title: "音樂台".to_string(), channel: "MU".to_string() },
            ChannelInfo { title: "影視台".to_string(), channel: "VI".to_string() },
            ChannelInfo { title: "攝影台".to_string(), channel: "DC".to_string() },
            ChannelInfo { title: "汽車台".to_string(), channel: "TS".to_string() },
            // 生活
            ChannelInfo { title: "上班台".to_string(), channel: "WK".to_string() },
            ChannelInfo { title: "感情台".to_string(), channel: "LV".to_string() },
            ChannelInfo { title: "校園台".to_string(), channel: "SC".to_string() },
            ChannelInfo { title: "親子台".to_string(), channel: "BB".to_string() },
            ChannelInfo { title: "寵物台".to_string(), channel: "PT".to_string() },
            ChannelInfo { title: "健康台".to_string(), channel: "HL".to_string() },
            // 其他
            ChannelInfo { title: "站務台".to_string(), channel: "MB".to_string() },
            ChannelInfo { title: "電　台".to_string(), channel: "RA".to_string() },
            ChannelInfo { title: "活動台".to_string(), channel: "AC".to_string() },
            ChannelInfo { title: "買賣台".to_string(), channel: "BS".to_string() },
            ChannelInfo { title: "直播台".to_string(), channel: "JT".to_string() },
            ChannelInfo { title: "成人台".to_string(), channel: "AU".to_string() },
            ChannelInfo { title: "考古台".to_string(), channel: "OP".to_string() },
        ]
    }

    /// Build channel lookup map
    fn build_channel_map(channels: &[ChannelInfo]) -> HashMap<String, usize> {
        channels.iter()
            .enumerate()
            .map(|(i, ch)| (ch.channel.clone(), i))
            .collect()
    }
}

impl Default for ChannelService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let service = ChannelService::new();
        assert_eq!(service.get_all_channels().len(), 42);
    }

    #[test]
    fn test_get_channel() {
        let service = ChannelService::new();
        let channel = service.get_channel("BW");
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().title, "吹水台");

        let invalid = service.get_channel("INVALID");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_is_valid_channel() {
        let service = ChannelService::new();
        assert!(service.is_valid_channel("BW"));
        assert!(service.is_valid_channel("HT"));
        assert!(!service.is_valid_channel("INVALID"));
    }

    #[test]
    fn test_filter_channels() {
        let service = ChannelService::new();
        let results = service.filter_channels("台");
        assert!(!results.is_empty());

        let empty = service.filter_channels("");
        assert!(empty.is_empty());

        let specific = service.filter_channels("吹水");
        assert!(!specific.is_empty());
    }

    #[test]
    fn test_get_channel_by_index() {
        let service = ChannelService::new();
        let channel = service.get_channel_by_index(1);
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().channel, "BW");

        let invalid = service.get_channel_by_index(100);
        assert!(invalid.is_none());
    }
}
