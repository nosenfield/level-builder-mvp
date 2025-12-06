# Update Filemap

Update an existing `.filemap.json` to reflect current directory contents.

## Usage

```
/filemap-update <directory-path>
```

## Steps

1. **Read Existing Filemap**:
   - Read `<directory>/.filemap.json`
   - If not exists, suggest `/filemap-create` instead

2. **Compare with Directory**:
   - List current files in directory
   - Identify additions (files not in filemap)
   - Identify removals (filemap entries for missing files)
   - Identify unchanged files

3. **Update Filemap**:
   - Add entries for new files (infer descriptions)
   - Remove entries for deleted files
   - Preserve descriptions for unchanged files
   - Update `d` if subdirectories changed

4. **Write Updated File**:
   - Overwrite `<directory>/.filemap.json`
   - Validate JSON syntax
   - Report changes

## Output Format

```markdown
## Filemap Updated

**Directory**: `<path>`

### Changes
**Added** (<count>):
- `new-file.ts`: "Description"

**Removed** (<count>):
- `deleted-file.ts`

**Unchanged** (<count>):
- `existing-file.ts`

### Updated Filemap
```json
{...}
```
```

## Notes

- Preserves existing descriptions (doesn't regenerate all)
- Only adds/removes entries for actual file changes
- Run `/filemap-validate` after to verify accuracy

## Excluded Directories

Do NOT update filemaps for these root directories (documentation/metadata, not code):
- `scripts/`
- `_docs/`
- `_context-summaries/`
- `tests/`
- `thoughts/`
