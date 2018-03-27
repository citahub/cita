# CITA

Visit the documentation site: <https://cryptape.github.io/cita/>

## Adding a github remote

```shell
$ git remote add upstream git@github.com:cryptape/cita.git
# Set a new remote
$ git remote -v
# Verify new remote
upstream    git@github.com:cryptape/cita.git (fetch)
upstream    git@github.com:cryptape/cita.git (push)
```

## Building the documentation site

```shell
pip install -r requirements.txt
mkdocs build
```

You can also use the `mkdocs serve` command to view the site on localhost, and live reload whenever you save changes.

## Redeploying the documentation site

mkdocs gh-deploy
