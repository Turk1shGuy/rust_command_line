pub fn help() {
    println!(r#"

CommandLine: Simple commands
============================

Commands:

    - cl echo <args>              :     Print args
    - cl read <args>              :     Read files <args>
    - cl list <dir>               :     List directories
    - cl find <dir> <type> <name> :     Find files or directories
    - cl grep <content> <input>   :     Search <content> in the <input>
    - cl clear                    :     Clear command line
    - cl mkdir                    :     Make a directory
    - cl pwd                      :     Prints working directory
    - cl rm                       :     Removes file or directory
    - cl whoami                   :     Prints current user

"#)
}
