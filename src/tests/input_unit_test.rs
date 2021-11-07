use crate::input;

/*
    @input: user typed "init" command (valid)
    @expect: method Initialize calls method ProcessInit
*/
#[test]
fn TestInitialize_GivenValidCommand();

/*
    @input: user typed "hello" command (invalid)
    @expect: method Initialize calls PrintError in display module
*/
#[test]
fn TestInitialize_GivenInvalidCommand();

/*
    @input: "help" command (valid)
    @expect: method ProcessHelp prints information about available commands
*/
#[test]
fn TestProcessHelp_GivenValidCommand();

/*
    @input: "help me" command (invalid)
    @expect: method ProcessHelp calls PrintError in display module
*/
#[test]
fn TestProcessHelp_GivenInvalidCommand();

/*
    @input: "init" command (valid)
    @expect: method ProcessInit calls method in repository module 
             to create an empty repository
*/
#[test]
fn TestProcessInit_GivenValidCommand();

/*
    @input: "init pls" command (invalid)
    @expect: method ProcessInit calls PrintError in display module
*/
#[test]
fn TestProcessInit_GivenInvalidCommand();

/*
    @input: "clone [valid url]" command (valid)
    @expect: method ProcessClone calls method in repository module
             to create an empty repository
*/
#[test]
fn TestProcessClone_GivenValidCommand();

/*
    @input: "clone" command (invalid)
    @expect: method ProcessClone calls PrintError in display module
*/
#[test]
fn TestProcessClone_GivenInvalidCommand();

/*
    @input: "add [valid file]" command (valid)
    @expect: method ProcessAdd calls method in repository module 
             to add specific files that user want to track
*/
#[test]
fn TestProcessAdd_GivenValidCommand();

/*
    @input: "add @#$RA" command (invalid)
    @expect: method ProcessAdd calls PrintError in display module
*/
#[test]
fn TestProcessAdd_GivenInvalidCommand();

/*
    @input: "remove [valid file]" command (valid)
    @expect: method ProcessRemove calls method in repository module 
             to remove specific files from tracking list
*/
#[test]
fn TestProcessRemove_GivenValidCommand();

/*
    @input:  "remove @#$abc^xyz" command (invalid)
    @expect: method ProcessRemove calls PrintError in display module
*/
#[test]
fn TestProcessRemove_GivenInvalidCommand();

/*
    @input: "status" command (valid)
    @expect: method ProcessStatus calls method in repository module 
             to check the current status of current repository
*/
#[test]
fn TestProcessStatus_GivenValidCommand();

/*
    @input: "status code.java" command (invalid)
    @expect: method ProcessStatus calls PrintError in display module
*/
#[test]
fn TestProcessStatus_GivenInvalidCommand();

/*
    @input: "heads" command (valid)
    @expect: method ProcessHeads calls method in repository module 
             to show the current heads
*/
#[test]
fn TestProcessHeads_GivenValidCommand();

/*
    @input: "heads goldfish" command (invalid)
    @expect: method ProcessHeads calls PrintError in display module
*/
#[test]
fn TestProcessHeads_GivenInvalidCommand();

/*
    @input: "diff [rev1] [rev2]" command (valid)
    @expect: method ProcessDiff calls method in repository module 
             to check the changes between revisions
*/
#[test]
fn TestProcessDiff_GivenValidCommand();

/*
    @input: "diff diff" command (invalid)
    @expect: method ProcessDiff calls PrintError in display module
*/
#[test]
fn TestProcessDiff_GivenInvalidCommand();

/*
    @input: "cat [file]" command (valid)
    @expect: method ProcessCat calls method in repository module 
             to inspect a file of a given revision
*/
#[test]
fn TestProcessCat_GivenValidCommand();

/*
    @input: "cat meow meow" command (invalid)
    @expect: method ProcessCat calls PrintError in display module
*/
#[test]
fn TestProcessCat_GivenInvalidCommand();

/*
    @input: "checkout [rev]" command (valid)
    @expect: method ProcessCheckout calls method in repository module 
             to check out a specific revision
*/
#[test]
fn TestProcessCheckout_GivenValidCommand();

/*
    @input: "checkout with cash" command (invalid)
    @expect: method ProcessCheckout calls PrintError in display module
*/
#[test]
fn TestProcessCheckout_GivenInvalidCommand();

/*
    @input: "commit" command (valid)
    @expect: method ProcessCommit calls method in repository module 
             to commit changes and create a new revision
*/
#[test]
fn TestProcessCommit_GivenValidCommand();

/*
    @input: "commit goldfish" command (invalid)
    @expect: method ProcessCommit calls PrintError in display module
*/
#[test]
fn TestProcessCommit_GivenInvalidCommand();

/*
    @input: "log" command (valid)
    @expect: method ProcessLog calls method in repository module 
             to view the change log
*/
#[test]
fn TestProcessLog_GivenValidCommand();

/*
    @input: "log goldfish" command (invalid)
    @expect: method ProcessLog calls PrintError in display module
*/
#[test]
fn TestProcessLog_GivenInvalidCommand();

/*
    @input: "merge [rev1] [rev2]" command (valid)
    @expect: method ProcessMerge calls method in repository module 
             to merge two revisions
*/
#[test]
fn TestProcessMerge_GivenValidCommand();

/*
    @input: "merge sort" command (invalid)
    @expect: method ProcessMerge calls PrintError in display module
*/
#[test]
fn TestProcessMerge_GivenInvalidCommand();

/*
    @input: "pull [repo]" command (valid)
    @expect: method ProcessPull calls method in repository module 
             to pull the changes from another repository
*/
#[test]
fn TestProcessPull_GivenValidCommand();

/*
    @input: "pull and bear" command (invalid)
    @expect: method ProcessPull calls PrintError in display module
*/
#[test]
fn TestProcessPull_GivenInvalidCommand();

/*
    @input: "pull [repo]" command (valid)
    @expect: method ProcessPush calls method in repository module 
             to push changes into another repository
*/
#[test]
fn TestProcessPush_GivenValidCommand();

/*
    @input: "pull and bear" command (invalid)
    @expect: method ProcessPush calls PrintError in display module
*/
#[test]
fn TestProcessPush_GivenInvalidCommand();



