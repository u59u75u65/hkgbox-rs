//! Command-line argument parsing
//!
//! This module provides simple command-line argument parsing for the application.
//! It supports selecting different forum services (HKGolden, LIHKG, etc.)

use std::env;

/// Supported forum services
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForumService {
    /// HKGolden (default)
    Hkgolden,
    /// LIHKG
    Lihkg,
}

impl ForumService {
    /// Parse a string into a ForumService
    ///
    /// # Arguments
    /// * `s` - String representation of the service
    ///
    /// # Returns
    /// The corresponding ForumService variant, or None if not recognized
    ///
    /// # Examples
    /// ```
    /// use hkg::cli::ForumService;
    ///
    /// assert_eq!(ForumService::from_str("hkgolden"), Some(ForumService::Hkgolden));
    /// assert_eq!(ForumService::from_str("lihkg"), Some(ForumService::Lihkg));
    /// assert_eq!(ForumService::from_str("invalid"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hkgolden" | "hkg" => Some(ForumService::Hkgolden),
            "lihkg" | "連登" => Some(ForumService::Lihkg),
            _ => None,
        }
    }

    /// Get the display name of the service
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            ForumService::Hkgolden => "HKGolden",
            ForumService::Lihkg => "LIHKG",
        }
    }

    /// Get the CLI argument value for the service
    #[must_use]
    pub const fn cli_value(&self) -> &str {
        match self {
            ForumService::Hkgolden => "hkgolden",
            ForumService::Lihkg => "lihkg",
        }
    }
}

impl std::fmt::Display for ForumService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Parsed command-line arguments
#[derive(Debug, Clone)]
pub struct Args {
    /// Forum service to use
    pub service: ForumService,
}

impl Args {
    /// Parse command-line arguments from environment
    ///
    /// # Returns
    /// Parsed arguments or an error message
    ///
    /// # Examples
    /// ```
    /// use hkg::cli::Args;
    /// use hkg::cli::ForumService;
    ///
    /// // Note: Args::parse() reads from actual command-line arguments
    /// // In real usage: let args = Args::parse();
    ///
    /// // For testing, you can check the service parsing:
    /// assert_eq!(ForumService::from_str("hkgolden"), Some(ForumService::Hkgolden));
    /// ```
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        // Default to HKGolden if no arguments provided
        if args.len() < 2 {
            return Ok(Args {
                service: ForumService::Hkgolden,
            });
        }

        // Parse arguments
        let mut service = ForumService::Hkgolden;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--service" | "-s" => {
                    if i + 1 >= args.len() {
                        return Err(format!("Missing value for {}", args[i]));
                    }
                    let service_str = &args[i + 1];
                    match ForumService::from_str(service_str) {
                        Some(s) => service = s,
                        None => {
                            return Err(format!(
                                "Unknown service '{}'. Supported services: hkgolden, lihkg",
                                service_str
                            ))
                        }
                    }
                    i += 2;
                }
                "--help" | "-h" => {
                    return Err("HELP".to_string());
                }
                arg => {
                    return Err(format!("Unknown argument: {}", arg));
                }
            }
        }

        Ok(Args { service })
    }

    /// Print usage information
    pub fn print_usage() {
        println!("Usage: hkg [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -s, --service <SERVICE>   Forum service to use (default: hkgolden)");
        println!("                             Supported: hkgolden, lihkg");
        println!("  -h, --help                 Print this help information");
        println!();
        println!("Examples:");
        println!("  hkg                        # Use HKGolden (default)");
        println!("  hkg --service hkgolden     # Use HKGolden explicitly");
        println!("  hkg --service lihkg        # Use LIHKG");
        println!("  hkg -s lihkg               # Use LIHKG (short form)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forum_service_from_str() {
        assert_eq!(ForumService::from_str("hkgolden"), Some(ForumService::Hkgolden));
        assert_eq!(ForumService::from_str("hkg"), Some(ForumService::Hkgolden));
        assert_eq!(ForumService::from_str("HKGOLDEN"), Some(ForumService::Hkgolden));

        assert_eq!(ForumService::from_str("lihkg"), Some(ForumService::Lihkg));
        assert_eq!(ForumService::from_str("連登"), Some(ForumService::Lihkg));
        assert_eq!(ForumService::from_str("LIHKG"), Some(ForumService::Lihkg));

        assert_eq!(ForumService::from_str("invalid"), None);
        assert_eq!(ForumService::from_str(""), None);
    }

    #[test]
    fn test_forum_service_display() {
        assert_eq!(ForumService::Hkgolden.name(), "HKGolden");
        assert_eq!(ForumService::Lihkg.name(), "LIHKG");

        assert_eq!(ForumService::Hkgolden.cli_value(), "hkgolden");
        assert_eq!(ForumService::Lihkg.cli_value(), "lihkg");

        assert_eq!(format!("{}", ForumService::Hkgolden), "HKGolden");
        assert_eq!(format!("{}", ForumService::Lihkg), "LIHKG");
    }
}
