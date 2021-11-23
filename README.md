# Legit [![legit](https://github.com/RandyMcMillan/legit/actions/workflows/automate.yml/badge.svg)](https://github.com/RandyMcMillan/legit/actions/workflows/automate.yml)

Legit is a tool for generating git commits with a custom commit hash prefix, like "000000".

As an example, take a look at a few of the commits in this repo.

### Usage

```shell
git clone https://github.com/RandyMcMillan/legit.git
cd legit && make legit
cd ~/Projects/your--project-repo
git add README.md
legit ./ -m "Add a README" -p "000000"
```

## Warning

__LEGIT WILL REVERT ANY UNSTAGED MODIFIED FILES IN YOUR REPOSITORY.  YOU WILL LOSE DATA IF YOU DO NOT REMEMBER THIS.__

--