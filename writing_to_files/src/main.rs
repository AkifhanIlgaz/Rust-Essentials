use std::fs;

fn main() {
    // J.F Kennedy
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things\n");
    speech.push_str("not because they are easy\n");
    speech.push_str("but because they are hard.");

    /*
        We can write to the files in a Rust program.

        For example, you want to generate Ethereum addresses that includes multiple 0's in it.
        Whenever you find one, you will write the corresponding private key of it to a file

        fs module enables us to do that.

        We want to save the Kennedy's Moon speech. Let's do it
    */

    /*
        Arguments of write()

        First Argument => path
            The path of the file that we want to write
            If there is no such a file creates one
            If there is a file in that path over-writes it !!
        Second Argument => contents
            Contents that we want to write
            All at once
    */

    fs::write("speech.txt", speech);
}
