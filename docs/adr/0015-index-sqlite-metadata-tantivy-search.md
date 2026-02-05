# ADR-0015: Index architecture — SQLite for metadata, Tantivy for full-text search

## Status

Accepted

## Context

Fracta needs an index layer to support:
1. **Fast file listing** — list directory contents without disk traversal
2. **Metadata queries** — filter by tags, date, area, etc.
3. **Full-text search** — search document content with good CJK (Chinese/Japanese/Korean) support
4. **Incremental updates** — re-index only changed files

The original plan (ENGINEERING.md §6.2) was to use SQLite FTS5 for everything. However, FTS5 has significant limitations:
- **Poor CJK tokenization**: `unicode61` tokenizer splits by character, not by word. "机器学习" becomes ["机", "器", "学", "习"], causing noisy search results.
- **No fuzzy search**: typo tolerance requires exact matches.
- **Limited ranking**: BM25 parameters are not tunable.

Tantivy is a Rust-native full-text search engine inspired by Lucene:
- **2x faster than Lucene** in search latency benchmarks.
- **Good CJK support** via `tantivy-jieba` (Chinese), `lindera` (Japanese/Korean).
- **Fuzzy search**, phrase queries, faceting built-in.
- **BM25 with tunable parameters**.

The trade-off: Tantivy stores its own index files (not embedded in SQLite), adding architectural complexity.

## Decision

Use a **two-layer index architecture**:

1. **SQLite** (`index.sqlite`) — metadata and file registry
   - `files` table: path, mtime, size, scope, content_hash
   - `metadata` table: extracted front matter fields (title, tags, date, area)
   - Fast structural queries: `WHERE tag = 'rust' AND date > '2025-01-01'`
   - Incremental updates via mtime comparison

2. **Tantivy** (`search/` directory) — full-text search index
   - Indexes plain text extracted from Markdown documents
   - Chinese tokenization via `tantivy-jieba`
   - BM25 ranking, phrase search, fuzzy matching
   - Linked to SQLite via `path` field

### Query routing

| Query type | Engine | Example |
|------------|--------|---------|
| List files in directory | SQLite | `SELECT * FROM files WHERE parent = ?` |
| Filter by metadata | SQLite | `WHERE tags LIKE '%rust%' AND area = 'library'` |
| Full-text search | Tantivy | `query.parse("机器学习")` |
| Combined (search + filter) | Tantivy → SQLite | Search first, then filter results via SQLite |

### On-disk layout

```
.fracta/cache/
├── index.sqlite          # Metadata + file registry
└── search/               # Tantivy index directory
    ├── meta.json
    ├── .managed.json
    └── <segment files>
```

Both are cache-layer artifacts: deleting them triggers a full rebuild from SOT (filesystem + Markdown files).

## Consequences

### Benefits
- **Better search quality**: intelligent CJK tokenization, fuzzy search, tunable ranking
- **Better performance**: Tantivy is faster for full-text; SQLite is faster for structural queries
- **Separation of concerns**: each tool does what it's best at
- **Future-ready**: Tantivy ecosystem supports vector search extensions

### Costs
- **Two index systems**: slightly more code and two sets of files
- **Increased binary size**: ~5MB for Tantivy + jieba dictionary
- **Increased compile time**: ~30s additional

### Migration
- Previous ENGINEERING.md stated "SQLite FTS5" — this ADR supersedes that for full-text search
- SQLite remains for metadata; FTS5 is not used

## Alternatives considered

### 1. SQLite FTS5 only
- Rejected: CJK tokenization is poor; no fuzzy search; limited ranking control
- Would require external tokenizer integration (complex) or accepting degraded search quality

### 2. Tantivy only (no SQLite)
- Rejected: structural queries (list files, filter by tag) would require indexing all metadata in Tantivy
- Tantivy's update model (delete + reindex) is heavier than SQLite for metadata changes
- SQL is simpler for structural queries

### 3. Meilisearch / Typesense (external service)
- Rejected: violates local-first principle; requires running a separate server process

### 4. SQLite + external Chinese tokenizer (e.g., jieba via custom tokenizer)
- Considered but rejected: FTS5 custom tokenizers are complex to implement correctly
- Tantivy + tantivy-jieba is a cleaner, better-supported solution

## References

- [Tantivy GitHub](https://github.com/quickwit-oss/tantivy)
- [tantivy-jieba](https://github.com/jiegec/tantivy-jieba) — Chinese tokenizer
- [SQLite FTS5](https://www.sqlite.org/fts5.html)
