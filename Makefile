build:
	cargo build --release

install:
	install -Dm 0755 -t /usr/local/bin/ target/release/alacritty-theme-switcher
	install -Dm 0644 -t /usr/local/share/bash-completion/completions/ dist/completions/bash/completions/alacritty-theme-switcher
	install -Dm 0644 -t /usr/local/share/zsh/site-functions/ dist/completions/zsh/site-functions/_alacritty-theme-switcher
