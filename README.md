# Financial Contracts in Rust

This project is a personal experiment to model financial contracts using combinators, implemented in Rust.
This work is inspired to [Composing contracts: an adventure in financial engineering
](https://www.microsoft.com/en-us/research/publication/composing-contracts-an-adventure-in-financial-engineering/)
and aims to be faithful to the original paper.

**It is not production-ready** and is subject to significant refactoring and changes.

Comments and suggestions are welcome.

## Overview

The project enables modeling of financial contracts using a combination of different operators (combinators). Contracts are represented as combinations of operations like `And`, `Or`, `Scale`, `Truncate`, and others.

### Available Combinators:

- `Zero`: Represents a contract with no value.
- `One`: Represents a contract that pays one unit of a specific currency.
- `Give`: Inverts the cash flows of a contract.
- `And`: Combines two contracts, evaluating them simultaneously.
- `Or`: Selects the contract with the maximum value.
- `Truncate`: Limits a contract to a specific date.
- `Then`: Executes one contract after the other has expired.
- `Scale`: Multiplies the value of a contract by an observable.
- `Get`: Retrieves the value of a contract at a specific time.
- `AnyTime`: Allows the contract to be exercised at any time (currently not implemented).


## Contributing

Contributions are welcome but keep in mind that this project is currently experimental. Feel free to open issues or pull requests for discussion.

## License

This project is licensed under the MIT License. See the [`LICENSE`](https://opensource.org/license/MIT) file for details.

---

**Disclaimer**: This project is for personal experimentation and learning purposes. It is not intended for production use, and the API may change significantly as development progresses.
