//! Favorites and History Module
//!
//! Manages search favorites and history.

use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A saved favorite search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub name: String,
    pub query: String,
    pub extensions: Option<Vec<String>>,
    pub fuzzy: bool,
    pub created_at: String,
}

/// Search history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub query: String,
    pub timestamp: String,
    pub results_count: usize,
}

/// Favorites storage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FavoritesStore {
    pub favorites: Vec<Favorite>,
    pub history: Vec<HistoryEntry>,
}

/// Get the favorites file path
fn get_favorites_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".codesearch_favorites.json")
    } else {
        PathBuf::from(".codesearch_favorites.json")
    }
}

/// Load favorites from file
pub fn load_favorites() -> FavoritesStore {
    let path = get_favorites_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(store) = serde_json::from_str(&content) {
                return store;
            }
        }
    }
    FavoritesStore::default()
}

/// Save favorites to file
pub fn save_favorites(store: &FavoritesStore) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_favorites_path();
    let content = serde_json::to_string_pretty(store)?;
    fs::write(path, content)?;
    Ok(())
}

/// Manage favorites and history
pub fn manage_favorites(
    list: bool,
    add: Option<String>,
    remove: Option<String>,
    clear: bool,
    history: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = load_favorites();

    if list {
        println!("{}", "⭐ Saved Favorites".cyan().bold());
        println!("{}", "─".repeat(20).cyan());

        if store.favorites.is_empty() {
            println!("{}", "No favorites saved yet.".dimmed());
        } else {
            for (i, fav) in store.favorites.iter().enumerate() {
                println!(
                    "  {}. {} - \"{}\"{}",
                    (i + 1).to_string().yellow(),
                    fav.name.green().bold(),
                    fav.query,
                    if fav.fuzzy { " (fuzzy)".dimmed().to_string() } else { "".to_string() }
                );
            }
        }
        return Ok(());
    }

    if let Some(query) = add {
        let name = format!("fav_{}", store.favorites.len() + 1);
        let favorite = Favorite {
            name: name.clone(),
            query: query.clone(),
            extensions: None,
            fuzzy: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        store.favorites.push(favorite);
        save_favorites(&store)?;
        println!("{}", format!("✅ Added '{}' to favorites as '{}'", query, name).green());
        return Ok(());
    }

    if let Some(name) = remove {
        let len_before = store.favorites.len();
        store.favorites.retain(|f| f.name != name);
        if store.favorites.len() < len_before {
            save_favorites(&store)?;
            println!("{}", format!("🗑️  Removed '{}' from favorites", name).yellow());
        } else {
            println!("{}", format!("Favorite '{}' not found", name).red());
        }
        return Ok(());
    }

    if clear {
        store.favorites.clear();
        save_favorites(&store)?;
        println!("{}", "🗑️  Cleared all favorites".yellow());
        return Ok(());
    }

    if history {
        println!("{}", "📜 Search History".cyan().bold());
        println!("{}", "─".repeat(20).cyan());

        if store.history.is_empty() {
            println!("{}", "No search history yet.".dimmed());
        } else {
            for entry in store.history.iter().rev().take(20) {
                println!(
                    "  {} \"{}\" ({} results)",
                    entry.timestamp.dimmed(),
                    entry.query.green(),
                    entry.results_count
                );
            }
        }
        return Ok(());
    }

    // Default: show help
    println!("{}", "Favorites Commands:".cyan().bold());
    println!("  --list    List all saved favorites");
    println!("  --add     Add a search to favorites");
    println!("  --remove  Remove a favorite by name");
    println!("  --clear   Clear all favorites");
    println!("  --history Show search history");

    Ok(())
}

/// Add an entry to search history
#[allow(dead_code)]
pub fn add_to_history(query: &str, results_count: usize) {
    let mut store = load_favorites();
    store.history.push(HistoryEntry {
        query: query.to_string(),
        timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string(),
        results_count,
    });

    // Keep only last 100 entries
    if store.history.len() > 100 {
        store.history = store.history.split_off(store.history.len() - 100);
    }

    let _ = save_favorites(&store);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorites_store_default() {
        let store = FavoritesStore::default();
        assert!(store.favorites.is_empty());
        assert!(store.history.is_empty());
    }

    #[test]
    fn test_favorite_creation() {
        let fav = Favorite {
            name: "test".to_string(),
            query: "fn main".to_string(),
            extensions: Some(vec!["rs".to_string()]),
            fuzzy: false,
            created_at: "2024-01-01".to_string(),
        };
        assert_eq!(fav.name, "test");
    }
}

