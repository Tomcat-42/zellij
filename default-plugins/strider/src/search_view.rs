
use crate::shared::calculate_list_bounds;
use zellij_tile::prelude::*;
use unicode_width::UnicodeWidthStr;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::file_list_view::FsEntry;

#[derive(Default, Debug)]
pub struct SearchView {
    pub search_results: Vec<SearchResult>,
    pub selected_search_result: usize,
}

impl SearchView {
    pub fn search_result_count(&self) -> usize {
        self.search_results.len()
    }
    pub fn update_search_results(&mut self, search_term: &str, files: &Vec<FsEntry>) {
        self.selected_search_result = 0;
        if search_term.is_empty() {
            self.search_results.clear();
        } else {
            let mut matches = vec![];
            let matcher = SkimMatcherV2::default().use_cache(true);
            for file in files {
                let name = file.name();
                if let Some((score, indices)) = matcher.fuzzy_indices(&name, search_term) {
                    matches.push(SearchResult::new(file.clone(), score, indices));
                }
            }
            matches.sort_by(|a, b| b.score.cmp(&a.score));
            self.search_results = matches;
        }
    }
    pub fn clear_and_reset_selection(&mut self) {
        self.search_results.clear();
        self.selected_search_result = 0;
    }
    pub fn move_selection_up(&mut self) {
        self.selected_search_result = self.selected_search_result.saturating_sub(1);
    }
    pub fn move_selection_down(&mut self) {
        if self.selected_search_result + 1 < self.search_results.len() {
            self.selected_search_result += 1;
        }
    }
    pub fn get_selected_entry(&self) -> Option<FsEntry> {
        self.search_results.get(self.selected_search_result).map(|s| s.entry.clone())
    }
    pub fn render(&mut self, rows: usize, cols: usize) {
        let (start_index, selected_index_in_range, end_index) = calculate_list_bounds(self.search_results.len(), rows, Some(self.selected_search_result));
        for i in start_index..end_index {
            if let Some(search_result) = self.search_results.get(i) {
                let is_selected = Some(i) == selected_index_in_range;
                let mut search_result_text = search_result.name();
                if search_result.is_folder() {
                    search_result_text.push('/');
                }
                let padding = " ".repeat(cols.saturating_sub(search_result_text.width()).saturating_sub(10));
                let mut text_element = if is_selected {
                    Text::new(format!(" <↓↑ TAB> {}{}", search_result_text, padding))
                        .color_indices(3, search_result.indices().iter().map(|i| i + 10).collect())
                        .color_range(3, 1..9)
                        .selected()
                } else {
                    Text::new(format!("          {}{}", search_result_text, padding))
                        .color_indices(3, search_result.indices().iter().map(|i| i + 10).collect())
                };
                if search_result.is_folder() {
                    text_element = text_element.color_range(0, ..);
                }
                print_text_with_coordinates(text_element, 0, i.saturating_sub(start_index) + 3, None, None);
            }
        }
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub entry: FsEntry,
    pub score: i64,
    pub indices: Vec<usize>
}

impl SearchResult {
    pub fn new(entry: FsEntry, score: i64, indices: Vec<usize>) -> Self {
        SearchResult {
            entry,
            score,
            indices
        }
    }
    pub fn name(&self) -> String {
        self.entry.name()
    }
    pub fn indices(&self) -> Vec<usize> {
        self.indices.clone()
    }
    pub fn is_folder(&self) -> bool {
        self.entry.is_folder()
    }
}
