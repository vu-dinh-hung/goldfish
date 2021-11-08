pub fn test_1_read_file(){
	asserteq!(dvcs.read_file(fileContainingHelloWorld), Ok("Hello World"))
}
pub fn test_2_write_file(){
	asserteq!(dvcs.write_file(invalidpath, somedata), Err("InvalidPathError"))
}
pub fn test_3_move_file(){

	asserteq!(dvcs.move_file(sourcepath, destpath, c=true), true)
	asserteq!(dvcs.read_file(sourcepath), dvcs.read_file(destpath))
}
pub fn test_4_remove_file(){
	asserteq!(dvcs.remove_file(path, r=true), true)
	pathprime = path + "/filename"
	asserteq!(dvcs.write_file(pathprime), Err("InvalidPathError"))
}

