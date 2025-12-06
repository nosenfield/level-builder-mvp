# Validate Filemap

Check if a `.filemap.json` accurately reflects its directory contents.

## Usage

```
/filemap-validate [directory-path]
```

If no path provided, validates all filemaps in project.

## Steps

1. **Find Filemaps**:
   - If path provided: validate that specific filemap
   - If no path: find all `.filemap.json` files in project

2. **For Each Filemap**:
   - Read the filemap JSON
   - List actual files in directory
   - Compare filemap entries with actual files

3. **Report Issues**:
   - Missing from filemap (files exist but not documented)
   - Stale entries (filemap has entries for missing files)
   - Subdirectory changes (new subdirs, removed subdirs)

4. **Generate Report**:
   - List all issues found
   - Suggest fixes
   - Report if filemap is valid

## Output Format

```markdown
## Filemap Validation Report

### Summary
- **Filemaps checked**: <count>
- **Valid**: <count>
- **Issues found**: <count>

### Results

#### `.filemap.json` (root)
✅ Valid

#### `frontend/src/.filemap.json`
⚠️ Issues found:
- **Missing from filemap**: `new-component.ts`
- **Stale entry**: `deleted-file.ts` (file not found)

**Fix**: Run `/filemap-update frontend/src`

#### `backend/src/.filemap.json`
✅ Valid

### Recommendations
- Run `/filemap-update frontend/src` to fix issues
- Consider creating filemap for: `scripts/` (no filemap, 5 files)
```

## Validation Rules

- Every file in directory should be in `f` (or consolidated from subdir)
- Every entry in `f` should correspond to existing file
- Subdirs with 3+ files should either have own filemap or be in `d`
- JSON must be syntactically valid

## Excluded Directories

Skip these root directories when validating (documentation/metadata, not code):
- `scripts/`
- `_docs/`
- `_context-summaries/`
- `tests/`
- `thoughts/`
