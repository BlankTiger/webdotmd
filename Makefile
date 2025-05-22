.PHONY: output
output:
	npx tailwindcss -i input.css -o ./assets/output.css
	cargo run -p webdotmd
	npx tailwindcss -i input.css -o ./output/output.css
