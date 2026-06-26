#!/bin/bash
cat << 'MD' > ./tmp/tools_review.md
# Tools Review

Here is the list of tools discovered in the donor codebase (`claude-code-main/src/tools/`):

MD

for tool_dir in $(ls ./tmp/claude-code-main/src/tools/ | grep Tool | sort); do
    tool_name=$(echo $tool_dir | sed 's/Tool$//')
    tool_file=$(ls ./tmp/claude-code-main/src/tools/$tool_dir/${tool_name}Tool.ts* 2>/dev/null | head -n 1)

    if [ -n "$tool_file" ]; then
        desc=$(grep -oP "searchHint:\s*'\K[^']+" "$tool_file" 2>/dev/null || grep -oP 'searchHint:\s*"\K[^"]+' "$tool_file" 2>/dev/null)
        if [ -z "$desc" ]; then
           desc=$(grep -A 2 "description()" "$tool_file" | grep "return" | sed -E "s/.*return ['\"]([^'\"]+)['\"].*/\1/" | head -n 1 | tr -d '\n')
        fi

        echo "- **${tool_dir}**: ${desc}" >> ./tmp/tools_review.md
    else
        echo "- **${tool_dir}**" >> ./tmp/tools_review.md
    fi
done

echo "Generated ./tmp/tools_review.md"
