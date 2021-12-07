# goldfish
CSC 253 DVCS implementation

## Structure of Goldfish

.goldfish <br>
|-> blobs <br>
|-> branches <br>
|-> commits <br>
|-> staging <br>
|-> *HEAD* <br>
|-> *tracked_files* <br>

**blobs folder:** contains blob file (copy of files at some moments)

**commits folder:** contains commits file
- Commit file stores 2 values:
  - parent: its parent commit
  - tracked files list: list of files and their blob ids to build up the project at that revision
- Commit file name is its commit id
- 
**staging folder:** staging area (it will only store files different from the last commit)

**branches folder:** for branching (but didn't have enough time to implement)

**HEAD file**: current commit id

**tracked_files file**: list of files and their blob ids for staging

## How things work (Core commands)
### 1. Init
- Check if the current folder is already goldfish project or not (by checking itself and its parent containing .goldfish folder or not)
- If already goldfish folder, error
- Otherwise create goldfish folder and create blobs folder, branches folder, commits folder, HEAD file, and tracked_files file inside it
### 2. Add <path>
For each file in path:
- Get content of the file and hash the file content
- Check tracked_files to get the last added file content hash (blob id) of the file
- Compare current hash with the last added hash, if they are the same, skip the file as it wasn't changed
- Otherwise, copy the file to staging area and update the list of tracked files to add/update the file

### 3. Remove <path>
- Remove path in staging (if path is a folder, delete the corresponding folder of path in staging). The corresponding of the file or folder may not exist in staging because of the optimization storing only files different from the last commit.
- For each file in path, update the list of tracked files to remove it

### 4. Commit
- Compare staging with HEAD (last commit). Check if list of tracked files of them are identical or not.
- If identical, nothing has changed so abort
- Otherwise, continue to commit
- Create blobs file for each file in staging
- Clean staging
- Create commit:
  - Include parent which is the HEAD
  - Copy current tracked file list
  - Write commit file
  - Update HEAD to be the just created commit

### 5. Status
Comparing staging with HEAD:
- Get list of tracked file for staging
- Get list of tracked file for HEAD
- Compare the 2 lists to find which files was added, removed, or changed

Comparing current WD with staging
- Get list of tracked file for staging
- Get list of files and their file content hash from current WD (exclude everything in .goldfish folder)
- Compare the 2 lists to find which files was added, removed, or changed

### 6. Checkout <commit>
- Get info for the commit from its commit file
- Copy the blob of each file in the commit's tracked file list to staging area
- Replace staging tracked file list with the commit's tracked file list
- Copy the staging area to the working path
- Clean staging

### 7. Clone <url>
- Check if there's any repository exists
- If not, download the content from specified URL using rsync
- Extract content from .goldfish folder

### 8. Heads
- Check if heads exists in the repository
- Print out the heads

### 9. Diff <commit> <commit>
- Check the difference between two files using longest common subsequence algorithm
- Iterate through each file in each revision and print out the diferrences

### 10. Cat <commit_id> <path>
- Find the revision based on revision id
- Find the file in the revision based on the file name
- Print out the content of the file

### 11. Log
- Check the history by going up in the commit tree
- Print out the history as we go up

### 12. Merge <commit_id>
- Compare the file difference between two commits
- For files that two commits have in common, generate the merged version for each file (by using diff algorithm)

### 13. Pull <url>
- Check if the URL contains a valid repository
- Fetch the content in the specified URL
- Merge with the current repository

### 14. Push <url>
- Check if the current directory is a valid repository
- Push to specified URL by using rsync