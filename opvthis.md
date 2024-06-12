## fork from deeean/sophia ?

## code - from deeean/sophia ?

```bash
# https://github.com/deeean/sophia.git
cd D:/code/nodejs/;
# degit deeean/sophia`#master sophia

degit deeean/sophia sophia
```

## open in vscode editor in cli

```powershell
code D:/code/nodejs/sophia;
```

## prepare vars

```powershell

$org="yors";$projname="sophia";

$codestore="d:/code/nodejs";

$rootws="$codestore/$projname"

$allowPackageLoc="docs,apps/*,packages/*,nana/*,nano/*,nanz/*,components/*";$disallowPackageLoc="";

$allowPackageLoc="packages/*";$disallowPackageLoc="docs,apps/*,nana/*,nano/*,nanz/*,components/*";

$repo="ymc-github/$projname";

$pkgsloc="packages";$name="nano";

$author="yemiancheng <ymc.github@gmail.com> (https://github.com/ymc-github)";

# switch dir to root workspace ? do.
# cd  $rootws

# open root workspace in vscode editor in cli ? do.
code $rootws
```

## code - get file from github repo

```powershell
$file=".github/workflows/CI.yml";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


$file="simple-test.js";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


$file="__test__/index.spec.ts";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


$file="benchmark/bench.ts";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/

$file=".eslintrc.yml";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/

yours rm package.json;
$file="package.json";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


$file=".gitattributes";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/

$file=".editorconfig";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/

$file=".prettierignore";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


$file=".taplo.toml";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/



$file="LICENSE";
yours download --url https://raw.githubusercontent.com/ymc-github/node-addon-fib-rs/main/$file --file $rootws/$file --ghproxy https://mirror.ghproxy.com/


```

## rw base-ify npm package

```powershell
# set your root workspace name for npm pakcage ? do
yours edit-json/name --file package.json --name "${projname}" --org "${org}" --workspace $rootws

# set version for your root workspace npm package ? do
yours edit-json  --file package.json --name "version" --value "0.1.0" --workspace $rootws

# set private for your root workspace npm package ? do
# yours edit-json  --file package.json --name "private" --value "true" --workspace $rootws

# set description for your root workspace npm package ? do
yours edit-json  --file package.json --name "description" --value "a project name ${projname}" --workspace $rootws

# set keywords for your root workspace npm package ? do
# yours edit-json/stra  --file package.json --name "keywords" --include "artist,${projname},mono" --workspace $rootws

# set license for your root workspace npm package ? do
yours edit-json  --file package.json --name "license" --value "MIT" --workspace $rootws


yours edit-json  --file package.json --name "author" --value "$author" --workspace $rootws

yours edit-json  --file package.json --name "repository.url" --value "https://github.com/$repo.git" --workspace $rootws

yours edit-json  --file package.json --name "bugs.url" --value "https://github.com/$repo/issues" --workspace $rootws

yours edit-json  --file package.json --name "homepage" --value "https://github.com/$repo/blob/main/#readme" --workspace $rootws

# set keys with order for your root workspace npm package ? do
yours edit-json/sort-keys --file package.json  --preset a --workspace $rootws

# add and set commit-msg for this task:
# git add package.json pnpm-workspace.yaml; git commit -m "build(mono): root workspace base-ify"
```

## set scripts format/lint/bench

```powershell
yours edit-json  --file package.json --name "scripts.format" --value "run-p format:prettier format:rs format:toml"  --workspace $rootws

yours edit-json  --file package.json --name "scripts.format:prettier" --value "prettier . -w" --workspace $rootws

yours edit-json  --file package.json --name "scripts.format:toml" --value "taplo format" --workspace $rootws

yours edit-json  --file package.json --name "scripts.format:rs" --value "cargo fmt" --workspace $rootws

yours edit-json  --file package.json --name "scripts.lint" --value "eslint . -c ./.eslintrc.yml" --workspace $rootws

yours edit-json  --file package.json --name "scripts.bench" --value "node -r @swc-node/register benchmark/bench.ts"  --workspace $rootws
```

## set napi name and other

```powershell
yours edit-json  --file package.json --name "napi.name" --value "sophia"  --workspace $rootws

yours edit-json  --file package.json --name "napi.triples.defaults" --value "false"  --workspace $rootws

# only for window 32 and 64
yours edit-json  --file package.json --name "napi.triples.additional" --method del --workspace $rootws

yours edit-json  --file package.json --name "napi.triples.additional[0]" --value "x86_64-pc-windows-msvc"  --workspace $rootws
```

## deps/build/test/lint/commit/tags/publish

## change napi pacakge name

```powershell
# install node_module packages with yarn:
yarn

# change napi pacakge name:
npx napi rename -n @yors/sophia

# build:
yarn build

# test:
yarn test
yarn bench


# lint:
yarn lint
yarn format


# github & gh & ghg-ify:
# ...
$repo="ymc-github/sophia";
$repo_desc="a node.js library for automating Windows applications";

$repo_uname=$repo -replace "-","_" -replace "/","_";
$repo_name=$repo  -replace ".*/","";
$repo_user=$repo  -replace "/.*","";

$email=git config user.email;

$repo_user;
$repo_name;

# public
gh repo create $repo_name --public --description "$repo_desc"
# private
# gh repo create $repo_name --private --description "$repo_desc"

# gh repo deploy-key list --repo $repo

# create deploy token
ssh-keygen -C "$email" -f $HOME/.ssh/gh_$repo_uname -t ed25519 -N '""'

# gh - upload github deploy
gh repo deploy-key add $HOME/.ssh/gh_${repo_uname}.pub --repo $repo -w --title deploy;

# set ssh key client (warn: next cmd will overide .ssh/config)
$txt=@"
Host github.com
    User git
    HostName github.com
    PreferredAuthentications publickey
    IdentityFile ~/.ssh/gh_${repo_uname}
"@

set-content -path $HOME/.ssh/config -value $txt
ssh -T git@github.com
# ssh -Tv git@github.com

# set workflow write mode in webui (todo):
# ...

# prepare npm toekn as secret for github workflow:
# ...
# list
# gh secret list --repo $repo

# put
gh secret set --repo $repo  -f D:\book\secret\npm.token.auto.md
# NPM_TOKEN=xx in npm.token.md

# commit
git init; git add . ; git commit -m "build(core): init"

# push:
# ...
# git remote -v
# git remote remove ghg
git remote add ghg git@github.com:$repo.git
git push -u ghg main
git push ghg main
# git push -u ghg main;
git push ghg main;


# publish:
# yarn r/un version
# yarn artticfacts
# yarn prepublishOnly
# ...

# tag & publish & release:
# ...
# git log --oneline

$ver="0.1.0";
$ver="1.0.0";
$ver="1.0.1";$tagdesc="put package home page"
git tag v$ver HEAD

# git tag -a v$ver -m "version $ver"

# push one tag to remote ghg
git push ghg v$ver

# push all tag to remote ghg
# git push ghg --tags

# git push ghg main

# del tag
git tag -d v$ver
# del remote tag
git push ghg :refs/tags/v$ver


# git HEAD / HEAD^ / HEAD~
# HEAD = HEAD~0 = HEAD^0

# git tag v$ver HEAD
git tag v$ver HEAD^1

git tag -a v$ver -m "version $ver, tag info"


# del
git tag -d v$ver;git push ghg :refs/tags/v$ver

# del release
gh release delete v$ver --repo $repo --yes

# add
# git tag v$ver HEAD -m "version $ver"; # case 1

# git tag v$ver HEAD -m "version $ver - $tagdesc"; # case 2

# git add . ;  git commit -m "version $ver";git tag v$ver HEAD -m "version $ver"; # case 3

# commit with version in msg and tag version ? do
git add . ;  git commit -m "$ver";git tag v$ver HEAD -m "$ver";

# $shcode='git log -1 --pretty=%B | grep "^[0-9]\+\.[0-9]\+\.[0-9]\+$"';sh -c "$shcode";
# check if version in cmtlog
$shcode="git log -1 --pretty=%B | grep '^[0-9]\+\.[0-9]\+\.[0-9]\+$'";sh -c "$shcode";


# push to github
git push ghg main ; git push ghg v$ver

# git push ghg main --tags

# list workflow
gh workflow list --repo $repo
# info workflow status
gh workflow view release  --repo $repo
# gh run list --workflow deploy.yml  --repo $repo

# gh workflow view ci  --repo $repo
# gh workflow view lint  --repo $repo

# list
gh release list --repo $repo
# https://cli.github.com/manual/gh_release_list

# del
gh release delete v$ver --repo $repo --yes;
```

## commit -x

```powershell
git add . ; git commit -m "build(core): init";
git add . ; git commit -m "build(core): add license file";
git add . ; git commit -m "build(core): dispatch manually";
git add . ; git commit -m "build(core): debug workflow";
git add . ; git commit -m "build(core): correct package.json repository.url";
# yarn install : ECONNREFUSED 127.0.0.1:4873
# https://github.com/salesagility/SuiteCRM-Core/issues/5
```

## opv this file

```powershell
$thistpoickey="sophia"
$thisfilekey="*${thistpoickey}*";

git add $thisfilekey;
yours git/commit --msg "docs(core): add note for $thistpoickey" --color blue

#  get,add
yours git/modified/file --include $thisfilekey --color blue
yours git/modified/file --include $thisfilekey --color blue --method put
yours git/commit --msg "docs(core): put note for $thisfilekey" --color blue

yours git/untracked/file --include $thisfilekey --color blue
yours git/untracked/file --include $thisfilekey --color blue --method put
yours git/untracked --msg "docs(core): put note for $thisfilekey" --color blue

# yours git/newed/file --color blue  -h
```
