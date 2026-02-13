# BorrowTrack

BorrowTrack is a simple CLI tool written in Rust to help you track borrowed and lent items among friends or within a group. It helps keep a transparent record of who has what and when it should be returned.

## Features

- **Add Users**: Keep a list of people who borrow or lend items.
- **Add Items**: Manage the catalog of items being tracked.
- **Create Loans**: Record when an item is lent out, including a due date.
- **Track Status**: Easily mark items as returned.
- **Search & List**: Search by user or item name, and view all active or overdue loans.

## Installation

To build from source, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/Anikogb/BorrowTrack.git
cd BorrowTrack
cargo build --release
```

The binary will be available at `target/release/borrowtrack`.

## Usage

### Adding Entities
```bash
# Add a user
borrowtrack add-user "Alice"

# Add an item
borrowtrack add-item "Book" "A very interesting novel"
```

### Managing Loans
```bash
# Create a loan (Owner ID, Borrower ID, Item ID, Days)
borrowtrack loan 1 2 1 7

# Mark a loan as returned (Loan ID)
borrowtrack return 1
```

### Searching and Listing
```bash
# List active loans
borrowtrack list

# List overdue loans
borrowtrack overdue

# Search for a user
borrowtrack search --user "Alice"

# Search for an item
borrowtrack search --item "Book"
```

## Storage

By default, BorrowTrack stores data in a `data.json` file in the current directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
