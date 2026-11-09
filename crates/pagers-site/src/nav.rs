//! Navigation tree built from the directory layout: a directory's `index.md`
//! becomes the group's own link, every other page a leaf. No configuration
//! file is needed — the file tree is the configuration.

use std::collections::HashMap;

use crate::site::Page;

/// One section of the sidebar: a directory of pages plus nested groups.
#[derive(Debug)]
pub struct Group {
    pub title: String,
    /// The index page's URL; `None` when the directory has no index page, in
    /// which case the group renders as a heading over its children.
    pub url: Option<String>,
    /// Indices into the site's page list, in navigation order.
    pub pages: Vec<usize>,
    pub groups: Vec<Group>,
}

/// The full navigation tree for a site.
#[derive(Debug, Default)]
pub struct Nav {
    /// Pages at the content root, landing page first.
    pub pages: Vec<usize>,
    pub groups: Vec<Group>,
}

impl Nav {
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty() && self.groups.is_empty()
    }
}

struct DirInfo {
    /// Non-index pages directly in this directory.
    pages: Vec<usize>,
    /// The directory's index page, when present.
    index: Option<usize>,
    /// Sort key: position of the earliest page in the subtree.
    key: usize,
}

/// Builds the navigation tree from the sorted page list. Indices refer into
/// `pages`, which must already be sorted for navigation. Every ancestor
/// directory of a page becomes a group, so deep trees never lose entries.
pub fn build(pages: &[Page]) -> Nav {
    let mut dirs: HashMap<String, DirInfo> = HashMap::new();
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();

    for (i, page) in pages.iter().enumerate() {
        if page.dir.is_empty() {
            continue;
        }
        // Register the page's directory and every ancestor.
        let mut dir = page.dir.as_str();
        loop {
            let entry = dirs.entry(dir.to_string()).or_insert_with(|| DirInfo {
                pages: Vec::new(),
                index: None,
                key: i,
            });
            entry.key = entry.key.min(i);
            if page.dir == dir {
                if page.is_index() {
                    entry.index = Some(i);
                } else {
                    entry.pages.push(i);
                }
            }
            match dir.rsplit_once('/') {
                Some((parent, _)) => dir = parent,
                None => break,
            }
        }
    }

    // Parent → child directory edges, for every registered dir.
    for dir in dirs.keys() {
        let parent = parent_dir(dir);
        children_of.entry(parent).or_default().push(dir.clone());
    }

    let mut nav = Nav {
        pages: pages
            .iter()
            .enumerate()
            .filter(|(_, p)| p.dir.is_empty())
            .map(|(i, _)| i)
            .collect(),
        groups: Vec::new(),
    };
    sort_root_pages(&mut nav.pages, pages);

    let mut top = children_of.remove("").unwrap_or_default();
    top.sort_by_key(|d| dirs[d].key);
    for dir in top {
        nav.groups.push(group_for(&dir, &dirs, &children_of, pages));
    }
    nav
}

fn group_for(
    dir: &str,
    dirs: &HashMap<String, DirInfo>,
    children_of: &HashMap<String, Vec<String>>,
    pages: &[Page],
) -> Group {
    let info = &dirs[dir];
    let (title, url) = match info.index {
        Some(i) => (pages[i].nav_title().to_string(), Some(pages[i].url.clone())),
        None => (
            crate::site::humanize_pub(dir.rsplit('/').next().unwrap_or(dir)),
            None,
        ),
    };
    let mut subdirs = children_of.get(dir).cloned().unwrap_or_default();
    subdirs.sort_by_key(|d| dirs[d].key);
    Group {
        title,
        url,
        pages: info.pages.clone(),
        groups: subdirs
            .iter()
            .map(|d| group_for(d, dirs, children_of, pages))
            .collect(),
    }
}

fn parent_dir(dir: &str) -> String {
    dir.rsplit_once('/')
        .map(|(p, _)| p.to_string())
        .unwrap_or_default()
}

/// Re-sorts root pages by `order` then name — the page list is already sorted,
/// but the landing page must come first regardless.
fn sort_root_pages(indices: &mut Vec<usize>, pages: &[Page]) {
    indices.sort_by(|&a, &b| {
        let (pa, pb) = (&pages[a], &pages[b]);
        match (pa.order, pb.order) {
            (Some(x), Some(y)) if x != y => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => pa.source.to_lowercase().cmp(&pb.source.to_lowercase()),
        }
    });
    if let Some(pos) = indices.iter().position(|&i| pages[i].url == "/") {
        let landing = indices.remove(pos);
        indices.insert(0, landing);
    }
}
