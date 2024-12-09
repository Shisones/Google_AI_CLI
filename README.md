# Google AI CLI

This project is a Rust wrapper for generating content using the [Generative Language API](https://generativelanguage.googleapis.com). It sends a request to the API with a provided text and receives a response that includes the generated content, usage metadata, and more. The wrapper is built using `reqwest` for HTTP requests and `serde` for serializing and deserializing JSON data.

## Features

- Sends a request to the Generative Language API.
- Prints the generated content, token usage, and optional citation metadata.
## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/Shisones/Google_AI_CLI.git
    cd Google_AI_CLI
    ```

3. Build the project:

    ```bash
    cargo build
    ```

4. Run the project:

    ```bash
    cargo run
    ```

## Usage

1. **API Key**: Insert your API key in a `.env` file in the root directory of the project. 
2. **Prompt**: You will  be prompted to input the text that you want to send to the Generative Language API.
3. The program will send the request, process the response, and display the generated content, citation metadata (if any), and token usage information.

## Contributing

Currently, the project is just made to simply utilize the API so that i can access Gemini 1.5 in the terminal.
I plan to add the following in the future:
- Better API Key storing process
- Different Model Support such as Gemini Experimental or Gemma 2b
- Better project structure

Feel free to fork the repository, make improvements, and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

