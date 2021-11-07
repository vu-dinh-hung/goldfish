pub fn read_file_test(){
	asserteq!(dvcs.read_file(fileContainingHelloWorld), Ok("Hello World"))
}
pub fn write_file_test(){
	asserteq!(dvcs.write_file(invalidpath, somedata), Err("InvalidPathError"))
}
pub fn move_file_test(){

	asserteq!(dvcs.move_file(sourcepath, destpath, c=true), true)
	asserteq!(dvcs.read_file(sourcepath), dvcs.read_file(destpath))
}
pub fn remove_file_test(){
	asserteq!(dvcs.remove_file(path, r=true), true)
	pathprime = path + "/filename"
	asserteq!(dvcs.write_file(pathprime), Err("InvalidPathError"))
}

