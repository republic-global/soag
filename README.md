## SOAG: Son Of A Git

SOAG is a repository management toolkit designed for Git. It provides a set of tools to facilitate the management of Git repositories.

### Usage

`GitHub` or `GitLab` access tokens are necessary for separating repos. To configure access tokens use the `config` option:

```
soag config --set-github-token <YOUR-ACCESS-TOKEN>
soag config --set-gitlab-token <YOUR-ACCESS-TOKEN>
```
Or use the interactive setup:
```
soag config -i
```

To separate a target into its own repository, use the following command:

```
soag separate <target> --github <NAME-YOUR-REPO>
soag separate <target> --gitlab <NAME-YOUR-REPO>
```

* **The `.soagconfig` file is needed**

- `<target>`: The target directory or repository to be separated.
- `--github/--gitlab`: (FLAG): This flags are to specify which remote SCM to use 
- `repo-name`: The name to use for the creation of the new repo

This will create a new GitHub repository with the provided name and will use it as a subtree of the current project.

### Benefits

- **Integrity in History**: Every step of the repository changes is preserved, ensuring integrity in the history.
- **Preservation of References**: Users can work on the separated repository from the local (parent) repository or in isolation. References to each other are maintained, allowing for efficient collaboration and tracking of isolated histories.

