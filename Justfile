check-publish:
    cd atomic_struct_core && cargo publish --dry-run --allow-dirty
    cd atomic_struct && cargo publish --dry-run --allow-dirty

publish: check-publish
    cd atomic_struct_core && cargo publish
    cd atomic_struct && cargo publish
    git tag v`cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.name=="atomic_struct") | .version'`
    git push --tags
