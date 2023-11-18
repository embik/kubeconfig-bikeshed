pub fn print_zsh_magic() {
    let zsh_str = include_str!("files/zsh/kbs.source");
    println!("{zsh_str}");
}
