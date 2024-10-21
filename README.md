# git-open-rs

`git-open-rs` is a command-line tool written in Rust (by the way) that opens your Git repository's remote page in your default web browser. It's a fast and convenient way to navigate to your repository or branch on platforms like GitHub, GitLab, and Bitbucket directly from the terminal. 

## Features

- [x] **Open Repository Homepage**: Quickly open the remote repository page.
- [ ] **Open Current Branch**: Navigate directly to the current branch page.
- [ ] **Custom Remotes**: Specify a remote other than `origin`.
- [ ] **Platform Detection**: Automatically detects and formats URLs for GitHub, GitLab, Bitbucket, and more.
- [ ] **Lightweight and Fast**: Minimal dependencies ensure quick performance.

## Installation

### Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs).

### Build from Source

```bash
# Clone the repository
git clone https://github.com/rivado-e/git-open-rs.git

# Change directory
cd git-open-rs

# Build the release version
cargo build --release

# (Optional) Install globally
cargo install --path .
```

## Usage

Basic command structure:

```bash
git open [OPTIONS]
```

### Options

- `-b`, `--branch`: Opens the current branch page.
- `-r`, `--remote <name>`: Specifies a remote other than `origin`.
- `-h`, `--help`: Prints help information.
- `-V`, `--version`: Prints version information.

### Examples

- **Open repository homepage**:

  ```bash
  git open
  ```

- **Open current branch page**:

  ```bash
  git open --branch
  ```

- **Use a different remote**:

  ```bash
  git open --remote upstream
  ```

## How It Works

1. **Retrieve Remote URL**: Extracts the remote URL using `git remote get-url`.
2. **Determine Platform**: Parses the URL to identify the hosting platform.
3. **Format URL**: Constructs the correct web URL for the repository or branch.
4. **Open Browser**: Uses the `open` command to launch the URL in your default web browser.

## Supported Platforms

- **GitHub**
- **GitLab**
- **Bitbucket**
- **Azure DevOps**
- **Custom Git Servers** (basic support)

## Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the Repository**: Click the "Fork" button at the top right of the repository page.
2. **Clone Your Fork**:

   ```bash
   git clone https://github.com/rivado-e/git-open-rs.git
   ```

3. **Create a Branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Changes**: Implement your feature or bug fix.
5. **Commit Changes**:

   ```bash
   git commit -am "Add your commit message here"
   ```

6. **Push to Your Fork**:

   ```bash
   git push origin feature/your-feature-name
   ```

7. **Open a Pull Request**: Navigate to your fork on GitHub and open a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

- **Author**: Rivaldo Edah
- **Email**: redah@umd.edu
- **GitHub**: [rivado-e](https://github.com/rivado-e)

## Acknowledgments

- Thanks to the contributors of the original [`git-open`](https://github.com/paulirish/git-open) project for the inspiration.

## Notes

- **Cross-Platform Compatibility**: Tested on macOS, Linux, and Windows.
- **Issues**: If you encounter any problems, please open an [issue](https://github.com/rivado-e/git-open-rs/issues).


