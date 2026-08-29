#!/bin/bash

# Releases tsify.
#
#     ./scripts/release.sh 0.6.0             do it
#     ./scripts/release.sh status  0.6.0     where it got to, without doing anything
#     ./scripts/release.sh check   0.6.0     are the two crates a coherent pair
#
# One command, because there is only one thing to decide. It works out where the
# release already is and does the next thing, so a run that stopped halfway can
# be started again with the same words.
#
# It stops for exactly one thing it cannot do: `main` requires a pull request
# and enforces that on administrators, so the version bump has to be merged by a
# person. Give it `--wait` and it will sit there until the merge happens and
# then carry on.
#
# `tsify` and `tsify-macros` go up as a pair, macro first, and the root has to
# be published against a floor that already requires the new macro. Getting that
# wrong fails nothing at the time; it ships a combination where a user with an
# existing lockfile keeps the old macro crate and stops compiling. The workspace
# cannot notice, because it always builds against the path dependency. That is
# what 0.5.7 nearly did.
#
# Publishing cannot be undone — a version on crates.io is never replaced — so
# that step, and only that step, asks before it happens.
#
# Credentials come from cargo's own configuration (`cargo login`, or
# CARGO_REGISTRY_TOKEN). Nothing here reads or stores them.

set -Eeuo pipefail

cd "$(dirname "$0")/.."

# ── Shared ───────────────────────────────────────────────────────────────────

api="https://crates.io/api/v1/crates"
index_timeout_seconds=600
user_agent="tsify-release-script (+https://github.com/madonoharu/tsify)"

# Set by `-y`. It does not reach the question before publishing.
assume_yes=${RELEASE_ASSUME_YES:-false}
phase="starting"
wait_for_merge=false

step() { phase="$1"; echo; echo "== $1"; }
pass() { echo "  ok    $1"; }
note() { echo "  note  $1"; }

# Where a crash leaves you is more useful than which line it happened on. Only
# crashes: a deliberate stop uses `exit`, which does not fire this. `-E` above is
# what makes it reach failures inside functions at all — measured: without it,
# `f() { false; }; f` runs no trap.
trap 'echo >&2; echo "error: failed while ${phase}" >&2; echo "  ./scripts/release.sh status $version  says where things stand" >&2' ERR

usage() {
    cat >&2 <<'EOF'
Releases tsify.

    ./scripts/release.sh <version>            do it
    ./scripts/release.sh status  <version>    where it got to, without doing anything
    ./scripts/release.sh check   <version>    are the two crates a coherent pair

    -y, --yes        take the default answer to every question except the
                     one before publishing, which is always asked
    -w, --wait       wait for the release pull request to be merged, then carry on
        --allow-dirty  let `check` run against an uncommitted tree
EOF
    exit 2
}

confirm() { # prompt
    if [ "$assume_yes" = true ]; then
        return 0
    fi
    printf '%s [y/N] ' "$1"
    read -r answer
    case "$answer" in y | Y | yes) return 0 ;; *) return 1 ;; esac
}

# crates.io answers 403 to a request that does not identify itself, so every
# call to it goes through here. Measured: without the header, 403; with it, 200
# for a version that exists and 404 for one that does not.
fetch() { curl -sS --connect-timeout 10 --max-time 120 -A "$user_agent" "$@"; }

# Does crates.io have this version at all? The web API answers as soon as the
# version row exists; the index, which is what cargo resolves from, is written
# later, so `in_index` below is a different question.
published() { # crate
    local code
    # `-w` prints a code even when curl fails, so the fallback would append a
    # second one — measured: "000000".
    code=$(fetch -o /dev/null -w '%{http_code}' "$api/$1/$version") || code=000
    case "$code" in
        200) return 0 ;;
        404) return 1 ;;
        # Anything else is unknown, and guessing would mean either publishing
        # over something or skipping a crate that is not there.
        000) echo "error: could not reach crates.io for $1 $version" >&2; exit 1 ;;
        *) echo "error: crates.io answered $code for $1 $version" >&2; exit 1 ;;
    esac
}

# Can cargo resolve it yet? Asked through cargo's own registry machinery rather
# than by building a sparse-index URL here. Measured: exit 0 for a version that
# exists, 101 for one that does not.
in_index() { # crate
    cargo info --registry crates-io "$1@$version" >/dev/null 2>&1
}

# Which commit a published crate was built from. `cargo package` writes
# .cargo_vcs_info.json into the crate, and it survives to crates.io — measured
# on the published tsify 0.5.8, whose recorded sha1 is the commit v0.5.8 points
# at. This is what makes "did this come from here?" a fact rather than a
# question for whoever is at the keyboard.
published_sha() { # crate — the commit, or nothing if it cannot be trusted
    # `cargo publish --allow-dirty` records the commit *and* `"dirty": true`;
    # the sha on its own would claim the crate came from that commit when it
    # did not. A missing or malformed sha is refused for the same reason.
    fetch -L "$api/$1/$version/download" \
        | tar -xzOf - "$1-$version/.cargo_vcs_info.json" 2>/dev/null \
        | jq -er 'select(.git.dirty != true) | .git.sha1
                  | select(type == "string" and test("^[0-9a-f]{40}$"))'
}

wait_for_index() { # crate
    local waited=0
    until in_index "$1"; do
        if [ "$waited" -ge "$index_timeout_seconds" ]; then
            echo "error: $1 $version has not reached the index after ${waited}s" >&2
            echo "       it is published; run this again once the index catches up" >&2
            exit 1
        fi
        sleep 10
        waited=$((waited + 10))
        echo "  ... ${waited}s"
    done
    pass "the index has $1 $version"
}

# ── check ────────────────────────────────────────────────────────────────────

version_failures=0
other_failures=0

# Two buckets, because only one of them is something the version bump fixes.
# Offering to rewrite the version when the real problem is a feature that is no
# longer forwarded would be worse than saying nothing.
version_fail() { echo "  FAIL  $1" >&2; version_failures=$((version_failures + 1)); }
other_fail() { echo "  FAIL  $1" >&2; other_failures=$((other_failures + 1)); }

check() { # [--allow-dirty]
    # One pass over the metadata. `cargo metadata` rather than `cargo tree` or
    # `cargo info`: the floor below is a dependency *requirement*, and those two
    # print the version that was resolved — which here is the path dependency,
    # the very thing that cannot see the problem. Only the JSON surfaces carry
    # `req`, so this reads it once and takes the rest from the same document.
    local meta facts root_version macro_version floor wasm js json
    meta=$(cargo metadata --no-deps --format-version 1)
    facts=$(jq -er '
        (.packages[] | select(.name == "tsify")) as $root
        | (.packages[] | select(.name == "tsify-macros")) as $macro
        | [ $root.version,
            $macro.version,
            ($root.dependencies[] | select(.name == "tsify-macros") | .req),
            ( ["wasm-bindgen", "js", "json"][] as $f
              | (($root.features[$f] // []) | index("tsify-macros/" + $f)) != null
                and ($macro.features | has($f)) )
          ] | @tsv' <<<"$meta") || { other_fail "could not read the workspace metadata"; return 1; }
    IFS=$'\t' read -r root_version macro_version floor wasm js json <<<"$facts"

    # One version, in all three places it is written.
    if [ "$root_version" = "$version" ] && [ "$macro_version" = "$version" ]; then
        pass "tsify and tsify-macros are both $version"
    else
        version_fail "the manifests say tsify=$root_version tsify-macros=$macro_version, not $version"
    fi

    # The floor. This is the 0.5.7 failure.
    if [ "${floor#^}" = "$version" ]; then
        pass "tsify's tsify-macros floor is $floor"
    else
        version_fail "tsify's tsify-macros floor is $floor, not ^$version — an existing lockfile could keep an older macro crate"
    fi

    # Every feature the root forwards has to exist on the other side. They agree
    # today; nothing checked that they still do.
    forwards() { # feature, whether it does
        if [ "$2" = true ]; then
            pass "feature $1 is forwarded to the macro crate"
        else
            other_fail "feature $1 is not forwarded to the macro crate"
        fi
    }
    forwards wasm-bindgen "$wasm"
    forwards js "$js"
    forwards json "$json"

    # What a user actually gets: `--dry-run` packages both crates and builds the
    # root against the *packaged* macro rather than the path one, which is the
    # only place the floor is exercised. It warns rather than fails when the
    # versions already exist, so it is safe to run at any time.
    local log
    # An explicit template: GNU mktemp requires at least three X's, so the
    # BSD-only `-t prefix` form fails outright on Linux.
    log=$(mktemp "${TMPDIR:-/tmp}/tsify-dry-run.XXXXXX")
    echo "  ...   cargo publish --workspace --dry-run ${1:-}"
    if ! cargo publish --workspace --dry-run ${1:-} >"$log" 2>&1; then
        other_fail "the dry run failed — see $log"
        tail -5 "$log" >&2
    else
        pass "the workspace packages and verifies"
        rm -f "$log"
    fi

    # What the packaged root depends on, read from the file rather than from a
    # resolution: a resolution drags in lockfile checksums that differ between a
    # dry run and the registry.
    #
    # Not checked: the `Unpacking tsify-macros ... (tmp-registry)` line in the
    # dry run. It looks like proof that the root was built against the packaged
    # macro, but cargo prints it only when the crate is not already unpacked in
    # the package cache — measured: two dry runs of the same clean tree, one
    # with the line and one without.
    local packaged section packaged_version
    packaged="target/package/tsify-$version/Cargo.toml"
    if [ "$root_version" != "$version" ]; then
        # `cargo package` writes under the version in the manifest, so there is
        # nothing at this path to look at yet. Counting that as a separate
        # failure would hide that the whole problem is one version bump.
        note "skipping the packaged manifest — the tree is not at $version yet"
    elif [ ! -f "$packaged" ]; then
        other_fail "$packaged does not exist — the dry run did not get far enough"
    else
        section=$(awk '/^\[dependencies\.tsify-macros\]/{f=1;next} /^\[/{f=0} f' "$packaged")
        packaged_version=$(sed -n 's/^version = "\(.*\)"$/\1/p' <<<"$section")
        if [ "$packaged_version" = "$version" ]; then
            pass "the packaged root asks for tsify-macros $packaged_version"
        else
            version_fail "the packaged root asks for tsify-macros '$packaged_version', not $version"
        fi
        if grep -q '^path = ' <<<"$section"; then
            other_fail "the packaged root still carries a path dependency on tsify-macros"
        else
            pass "the packaged root has no path dependency left"
        fi
    fi

    return $((version_failures + other_failures))
}

cmd_check() {
    step "Checking $version"
    if check "$@"; then
        echo
        echo "Coherent."
    else
        echo
        echo "$((version_failures + other_failures)) check(s) failed." >&2
        if [ "$other_failures" -eq 0 ]; then
            echo "All of them are about the version, which './scripts/release.sh $version' moves." >&2
        fi
        exit 1
    fi
}

# ── prepare ──────────────────────────────────────────────────────────────────

rewrite() {
    # Every pattern is checked before anything is written. `perl -pi` replaces
    # first and counts afterward, so a pattern that stopped matching would
    # otherwise leave the earlier files rewritten and the release half-prepared.
    grep -q '^version = "' Cargo.toml || { echo "error: Cargo.toml has no version line" >&2; exit 1; }
    grep -q '^version = "' tsify-macros/Cargo.toml || { echo "error: tsify-macros/Cargo.toml has no version line" >&2; exit 1; }
    grep -q '^tsify-macros = { path = "tsify-macros", version = "' Cargo.toml || { echo "error: Cargo.toml has no tsify-macros floor to move" >&2; exit 1; }
    grep -q '^tsify = "' README.md || { echo "error: README.md does not tell people which version to depend on" >&2; exit 1; }
    grep -q '^# tsify Changelog$' CHANGELOG.md || { echo "error: CHANGELOG.md does not start the way this expects" >&2; exit 1; }

    replaced() { # file, expression, expected count
        local count
        count=$(perl -0pi -e "BEGIN { \$c = 0 } \$c += $2; END { print STDERR \$c }" "$1" 2>&1 >/dev/null)
        if [ "$count" != "$3" ]; then
            echo "error: $1: expected $3 replacement(s), made ${count:-0}" >&2
            exit 1
        fi
        echo "  $1"
    }

    # Only the first `version = ` in each manifest, which is the package's own;
    # the dependency requirements come later in the file.
    replaced Cargo.toml 's/^version = "[^"]*"/version = "'"$version"'"/m' 1
    replaced tsify-macros/Cargo.toml 's/^version = "[^"]*"/version = "'"$version"'"/m' 1

    # The floor.
    replaced Cargo.toml 's/(tsify-macros = \{ path = "tsify-macros", version = ")[^"]*(")/${1}'"$version"'${2}/' 1

    # What the README tells people to put in their own manifest.
    replaced README.md 's/^(tsify = ")[^"]*(")/${1}'"$version"'${2}/m' 1

    # An empty section to write into, in the shape the file already uses. What a
    # release note has to say is not derivable from a diff.
    if grep -q "^## v$version\$" CHANGELOG.md; then
        echo "  CHANGELOG.md already has a v$version heading"
    else
        replaced CHANGELOG.md 's/^(# tsify Changelog\n)/${1}\n## v'"$version"'\n\n<!-- Write the release notes here: what changed, and what a user has to do about it. -->\n/' 1
    fi
}

prepare_release() {
    require_clean_main

    step "Rewriting to $version"
    rewrite

    # The rewrite is only worth committing if it made the pair coherent. The
    # tree is dirty now by construction, so the dry run is told to allow that.
    step "Checking the result"
    if ! check --allow-dirty; then
        echo
        echo "The rewrite did not make this coherent. Nothing has been committed;" >&2
        echo "the changes are in the working tree for you to look at." >&2
        return 1
    fi

    # The entry is the one part of a release a script cannot write, and writing
    # it now beats remembering to before the merge.
    if [ -n "${EDITOR:-}" ] && [ "$assume_yes" != true ]; then
        step "The CHANGELOG entry"
        if confirm "Open CHANGELOG.md in $EDITOR?"; then
            # `EDITOR` is a command line, not a program name: `code --wait` is
            # ordinary. Left unquoted so it splits the way a shell would.
            # shellcheck disable=SC2086
            $EDITOR CHANGELOG.md
        fi
    else
        note "write the CHANGELOG entry before merging"
    fi

    step "What changed"
    git --no-pager diff

    if ! confirm "Commit this on release/$version and open the pull request?"; then
        echo "Left in the working tree. Nothing committed."
        return 0
    fi

    if git ls-remote --exit-code --heads origin "release/$version" >/dev/null 2>&1; then
        echo "error: release/$version already exists on the remote" >&2
        echo "       finish or delete it rather than overwriting someone's edits" >&2
        return 1
    fi

    step "Opening the release PR"
    git switch -c "release/$version"
    git add Cargo.toml tsify-macros/Cargo.toml README.md CHANGELOG.md
    git commit -m "Release $version"
    git push --set-upstream origin "release/$version"

    if command -v gh >/dev/null 2>&1; then
        gh pr create --base main --head "release/$version" \
            --title "Release $version" \
            --body "Moves both crates to $version, along with the floor \`tsify\` puts on \`tsify-macros\`, the version the README tells people to depend on, and a CHANGELOG heading.

Once this is merged, \`./scripts/release.sh $version\` on main uploads both crates." \
            || note "the pull request could not be opened — open it by hand; the branch is pushed"
    else
        note "gh is not installed — open the pull request by hand"
    fi

    if [ "$wait_for_merge" = true ] && command -v gh >/dev/null 2>&1; then
        step "Waiting for the pull request to be merged"
        echo "  merge it and this carries on by itself; Ctrl-C is safe"
        # Waiting for a person has no useful ceiling, but waiting for `gh` does:
        # an expired login answers nothing, and nothing is neither MERGED nor
        # CLOSED, so the loop would run until someone noticed.
        local waited=0 state unreadable=0
        while :; do
            if state=$(gh pr view "release/$version" --json state --jq .state 2>/dev/null); then
                unreadable=0
            elif [ $((unreadable += 1)) -ge 20 ]; then
                echo "error: gh could not read the pull request in twenty tries" >&2
                echo "       the branch is pushed; merge it and run this again" >&2
                return 1
            else
                state=UNKNOWN
            fi
            case "$state" in
                MERGED) break ;;
                CLOSED) echo "error: the pull request was closed without merging" >&2; return 1 ;;
            esac
            sleep 15
            waited=$((waited + 15))
            [ $((waited % 60)) -eq 0 ] && echo "  ... ${waited}s"
        done
        pass "merged"

        step "Catching up with main"
        git switch main
        git pull --ff-only

        publish_release
        return 0
    fi

    cat <<EOF

main requires a pull request and enforces that on administrators, so this stops
here. Once it is merged:

    git switch main && git pull
    ./scripts/release.sh $version

(or pass --wait next time and it will sit through the merge itself)

EOF
}

# ── status ───────────────────────────────────────────────────────────────────

cmd_status() {
    local head tag_state next macro_state root_state state sha
    head=$(git rev-parse HEAD)

    step "Where $version stands"

    # One pass: the line printed for a crate and the decision made about it come
    # from the same observation, rather than from asking crates.io twice. The
    # state is what the decision reads — "unusable" is not "absent", and being
    # published without being in the index is not being finished.
    describe() { # crate — prints a line, leaves a word in $state
        if ! published "$1"; then
            state=absent
            echo "  $1: not published"
            return
        fi
        if ! sha=$(published_sha "$1"); then
            state=unusable
            echo "  $1: published, but its provenance is unreadable or dirty"
        elif [ "$sha" != "$head" ]; then
            state=foreign
            echo "  $1: published from ${sha:0:7}, which is not HEAD"
        elif in_index "$1"; then
            state=indexed
            echo "  $1: published from HEAD, in the index"
        else
            state=unindexed
            echo "  $1: published from HEAD, NOT yet in the index"
        fi
    }
    describe tsify-macros; macro_state=$state
    describe tsify; root_state=$state

    # The tag has to agree on both sides: a local tag on HEAD with origin
    # holding a different one is a release that cannot be finished here.
    local remote_tag local_tag
    local_tag=$(git rev-parse --verify --quiet "v$version^{}" || true)
    remote_tag=$(git ls-remote origin "refs/tags/v$version" "refs/tags/v$version^{}" | tail -1 | cut -f1)
    if [ -n "$remote_tag" ] && [ "$remote_tag" != "$head" ]; then
        tag_state="origin has v$version at ${remote_tag:0:7}, which is not HEAD"
    elif [ -n "$local_tag" ] && [ "$local_tag" != "$head" ]; then
        tag_state="v$version points at ${local_tag:0:7}, not HEAD"
    elif [ -n "$remote_tag" ]; then
        tag_state="v$version is pushed and points at HEAD"
    elif [ -n "$local_tag" ]; then
        tag_state="v$version is here but not pushed"
    else
        tag_state="v$version does not exist"
    fi
    echo "  tag: $tag_state"

    # What to do next follows from where the published crates came from, not
    # just from whether they exist: a version released from another commit is
    # finished, whatever this checkout says.
    case "$macro_state:$root_state" in
        absent:absent) next="./scripts/release.sh $version" ;;
        foreign:* | *:foreign) next="nothing — $version was released from another commit; release the next version instead" ;;
        unusable:* | *:unusable) next="nothing safe — look at what was published before doing anything else" ;;
        absent:*) next="nothing safe — tsify is published and tsify-macros is not, which this script cannot produce" ;;
        *:absent) next="./scripts/release.sh $version   (it resumes from the macro crate)" ;;
        indexed:indexed)
            if [ "$tag_state" = "v$version is pushed and points at HEAD" ]; then
                next="done"
            else
                next="./scripts/release.sh $version   (it only needs to tag)"
            fi
            ;;
        *) next="./scripts/release.sh $version   (it waits for the index, then tags)" ;;
    esac
    echo
    echo "  next: $next"
}

# ── publish ──────────────────────────────────────────────────────────────────

# Refuses to resume onto a crate that came from somewhere else. crates.io
# versions are immutable, so a macro crate published from a different tree can
# never be made to agree with this one at this version.
same_tree_or_stop() { # crate
    local sha
    sha=$(published_sha "$1" || true)
    if [ -z "$sha" ]; then
        echo "error: $1 $version is published but its provenance is not usable" >&2
        echo "       either it records no commit, or it was published from a dirty tree" >&2
        exit 1
    fi
    if [ "$sha" != "$(git rev-parse HEAD)" ]; then
        echo "error: $1 $version was published from ${sha:0:7}, not from HEAD" >&2
        echo "       the pair cannot be made to agree at $version; release the next one" >&2
        exit 1
    fi
    pass "$1 $version was published from this commit"
}

publish_release() {
    require_clean_main

    step "Checking $version"
    check || { echo; echo "Not publishing." >&2; exit 1; }

    step "What is already published"
    local macro_done=false root_done=false
    published tsify-macros && macro_done=true || true
    published tsify && root_done=true || true
    echo "  tsify-macros: $($macro_done && echo published || echo "not published")"
    echo "  tsify:        $($root_done && echo published || echo "not published")"

    if $macro_done; then
        same_tree_or_stop tsify-macros
    fi
    if $root_done; then
        same_tree_or_stop tsify
        if $macro_done; then
            note "both are already published from this commit"
            # A run that stopped after publishing has not necessarily seen them
            # reach the index, and a release is not finished until a downstream
            # build can resolve it.
            wait_for_index tsify-macros
            wait_for_index tsify
            tag_release
            return 0
        fi
        # tsify without tsify-macros is not something this script can produce.
        echo "error: tsify is published and tsify-macros is not" >&2
        return 1
    fi

    # Asked even under -y. Everything else here can be undone by hand; this
    # cannot, and a flag meant to keep a test moving should not be able to
    # publish.
    step "Ready to publish $version"
    echo "  This uploads to crates.io. A version there can never be replaced."
    printf 'Type the version to continue: '
    read -r confirm_version
    if [ "$confirm_version" != "$version" ]; then
        echo "Stopping." >&2
        exit 1
    fi

    if ! $macro_done; then
        step "Publishing tsify-macros $version"
        cargo publish -p tsify-macros
    fi

    # Even when resuming, the index has to carry the macro crate before the root
    # can resolve it — the earlier run may have stopped before it appeared.
    step "Waiting for the index to carry tsify-macros $version"
    wait_for_index tsify-macros

    step "Publishing tsify $version"
    cargo publish -p tsify

    # The root's own index entry is what a downstream build needs, so the
    # release is not finished until it is there.
    step "Waiting for the index to carry tsify $version"
    wait_for_index tsify

    tag_release
}

# ── tag ──────────────────────────────────────────────────────────────────────

tag_release() {
    step "Tagging"
    local head
    head=$(git rev-parse HEAD)
    if git rev-parse --verify --quiet "refs/tags/v$version" >/dev/null; then
        if [ "$(git rev-parse "v$version^{}")" != "$head" ]; then
            echo "error: the local tag v$version points at $(git rev-parse --short "v$version^{}"), not HEAD" >&2
            return 1
        fi
        note "v$version already exists here and points at HEAD"
    else
        git tag -a "v$version" -m "tsify $version"
    fi

    # When origin already has this tag pointing at the same commit, `git push`
    # reports it as nothing to do — measured. So a tag on a *different* commit
    # has to be caught here rather than left to the push. One query returns the
    # tag object and, for an annotated tag, the commit it peels to, in that
    # order, so the last line is the commit.
    local remote_tag
    remote_tag=$(git ls-remote origin "refs/tags/v$version" "refs/tags/v$version^{}" | tail -1 | cut -f1)
    if [ -n "$remote_tag" ] && [ "$remote_tag" != "$head" ]; then
        echo "error: origin already has v$version at ${remote_tag:0:7}, which is not HEAD" >&2
        return 1
    fi

    git push origin "v$version"
    pass "v$version pushed"

    cat <<EOF

Released $version. Still to do, by hand:

  - build something against the published crates rather than this tree
  - write the GitHub release notes from the CHANGELOG entry
  - answer the issues that were waiting on this version

If that build fails, the fix is a new version rather than a yank: yanking does
not reach a lockfile that has already resolved.
EOF
}

# ── The whole thing ──────────────────────────────────────────────────────────

# The package's own version is the first `version = ` in the manifest; reading
# it does not need a full `cargo metadata`, which `check` runs anyway.
manifest_version() {
    awk -F'"' '/^version = "/ { print $2; exit }' Cargo.toml
}

cmd_release() {
    # Moving to the version needs a merge, so that path either stops or, with
    # --wait, carries on into publishing by itself.
    if [ "$(manifest_version)" != "$version" ]; then
        prepare_release
    else
        publish_release
    fi
}

# ── Preconditions shared by the stages that change something ────────────────

require_clean_main() {
    step "Checking the working tree"

    local branch
    branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$branch" != "main" ]; then
        echo "error: on '$branch'; releases go out from main" >&2
        exit 1
    fi

    if [ -n "$(git status --porcelain)" ]; then
        echo "error: the working tree is not clean" >&2
        git status --short >&2
        exit 1
    fi

    git fetch --quiet origin main
    if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
        echo "error: HEAD is not origin/main — push or pull first" >&2
        exit 1
    fi

    pass "main, clean, and level with origin"
}

# ── Dispatch ─────────────────────────────────────────────────────────────────

command="release"
version=""
allow_dirty=""
for arg in "$@"; do
    case "$arg" in
        status | check) command="$arg" ;;
        -y | --yes) assume_yes=true ;;
        -w | --wait) wait_for_merge=true ;;
        --allow-dirty) allow_dirty="--allow-dirty" ;;
        -*) echo "error: unknown option $arg" >&2; exit 2 ;;
        # A word that is not a subcommand is the version. Taking a second one
        # would mean a mistyped `status` silently ran the release instead —
        # measured: `statsu 0.6.0` used to reach the release path.
        *)
            if [ -n "$version" ]; then
                echo "error: don't know what to do with '$arg'" >&2
                echo "       the only subcommands are 'status' and 'check'" >&2
                exit 2
            fi
            version="$arg"
            ;;
    esac
done

[ -n "$version" ] || usage
case "$version" in
    v*) echo "error: pass the version without the leading v" >&2; exit 2 ;;
esac
# A glob would let `1x.2y.3z`, `1.2.3-rc.1` and a trailing space through, and
# this ends up in a manifest, a tag and a published version.
if ! printf '%s' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "error: '$version' is not an x.y.z version" >&2
    exit 2
fi

case "$command" in
    release) cmd_release ;;
    check) cmd_check $allow_dirty ;;
    status) cmd_status ;;
esac
