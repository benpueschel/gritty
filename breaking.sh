#!/usr/bin/env bash

print_help() {
	NAME=$(basename $0)
	echo "Get the breaking changes between two tags"
	echo ""
	echo "Usage: $NAME <old_tag> [new_tag]"
	echo "  old_tag: The old tag to compare"
	echo "  new_tag: The optional new tag to compare. Defaults to the latest tag"
}

if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
	print_help
	exit 0
fi

if [ -z "$1" ]; then
	echo "Error: old_tag is required"
	print_help
	exit 1
fi

old_tag=$1
new_tag=$2

breaking=$(curl -s "https://raw.githubusercontent.com/benpueschel/gritty/main/CHANGELOG.md")

is_searching=false
if [ -z "$new_tag" ]; then
	is_searching=true
fi

is_breaking=false
echo "$breaking" | while read -r line; do
	case $line in
		"## [$new_tag]"*)
			is_searching=true
			;;
		"## [$old_tag]"*)
			is_searching=false
			break
			;;
		"### :boom:"*)
			is_breaking=true
			;;
		"##"*)
			is_breaking=false
			;;
		# Ignore empty lines
		"") ;;
		*)
			if $is_searching && $is_breaking; then
				echo $line
			fi
			;;
	esac

done
