# Create Filemap

Generate a new `.filemap.json` for a directory.

## Usage

```
/filemap-create <directory-path>
```

## Steps

1. **Verify Directory**:
   - Check directory exists
   - Check no existing `.filemap.json`
   - If exists, suggest `/filemap-update` instead

2. **Analyze Directory**:
   - List all files in directory
   - Identify subdirectories
   - For subdirs with <3 files: consolidate into parent `f`
   - For subdirs with 3+ files: add to `d` (suggest separate filemap)

3. **Generate Filemap**:
   - `m`: Directory path relative to project root
   - `p`: Infer purpose from file contents and names
   - `f`: Map each file to one-sentence description
   - `d`: Map each unconsolidated subdir to description
   - `x`: List key exports (for TypeScript/JavaScript modules)
   - `r`: Identify related files in other directories

4. **Write File**:
   - Create `<directory>/.filemap.json`
   - Validate JSON syntax
   - Report completion

## Output Format

```markdown
## Filemap Created

**Directory**: `<path>`
**Files Documented**: <count>
**Subdirectories**: <count> (consolidated: <count>, separate: <count>)

### Generated Filemap
```json
{...}
```

**Next Steps**:
- Review and adjust descriptions if needed
- Run `/filemap-validate <path>` to verify accuracy
```

## Consolidation Rules

- Subdirectory has <3 files → consolidate into parent's `f` with `subdir/file.ext` keys
- Subdirectory has 3+ files → add to `d`, suggest running `/filemap-create` for it
- Asset directories (images, sounds) → mention in `d`, don't list individual files

## Excluded Directories

Do NOT create filemaps for these root directories (documentation/metadata, not code):
- `scripts/`
- `_docs/`
- `_context-summaries/`
- `tests/`
- `thoughts/`
