## SOAG: Son Of A Git

SOAG is a repository management toolkit designed for Git. It provides a set of tools to facilitate the management of Git repositories.

### Usage

To separate a target into its own repository, use the following command:

`rep separate <target> [url]`


- `<target>`: The target directory or repository to be separated.
- `[url]` (optional): The URL of the remote repository where the newly created repository will be pushed. If not provided, a local repository will be created.

If no URL is provided, SOAG will create a `.rep/` directory where it will move the `target` and create a local repository. This allows users to work with the separated repository locally or push/pull changes as desired.

### Benefits

- **Integrity in History**: Every step of the repository changes is preserved, ensuring integrity in the history.
- **Preservation of References**: Users can work on the separated repository from the local (parent) repository or in isolation. References to each other are maintained, allowing for efficient collaboration and tracking of isolated histories.

