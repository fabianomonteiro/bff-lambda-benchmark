# Flask API Project

## Overview

This project is a REST API built with Flask that provides various utilities, including mathematical operations, JSON manipulation, string processing, data compression, and image generation.

The API is designed to showcase how to build and use Flask-based services with middleware for timing requests. Each endpoint focuses on a specific functionality, such as working with NumPy for math, handling JSON, and generating images using the Pillow library.

---

## Features

- **Middleware**: Tracks the execution time of requests and adds timing information to response headers.
- **Endpoints**:
  - Perform mathematical operations using NumPy.
  - Manipulate JSON data.
  - Search for patterns in strings using regular expressions.
  - Compress text data into Gzip format.
  - Generate images with custom text.

---

## Requirements

- Python 3.8 or higher
- `pip` for managing dependencies

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/fabianomonteiro/bff-lambda-benchmark.git
   cd bff-lambda-benchmark
   ```

2. Create a virtual environment:
   ```bash
   python -m venv venv
   source venv/bin/activate  # macOS/Linux
   .\venv\Scripts\activate   # Windows
   ```

3. Install dependencies:
   ```bash
   pip install flask numpy pillow
   ```

4. Run the application:
   ```bash
   python app.py
   ```

   The API will run locally on `http://127.0.0.1:5000`.

---

## API Documentation

### **1. `/math`**

Perform mathematical operations (sum or product) on an array of numbers.

- **Method**: `POST`
- **Payload**:
  ```json
  {
    "numbers": [1, 2, 3],
    "operation": "sum"  // Options: "sum" or "product"
  }
  ```
- **Response**:
  ```json
  {
    "result": 6
  }
  ```

**Example CURL**:
```bash
curl -X POST http://127.0.0.1:5000/math \
-H "Content-Type: application/json" \
-d '{"numbers": [1, 2, 3], "operation": "sum"}'
```

---

### **2. `/json`**

Create a JSON object from a given key-value pair.

- **Method**: `POST`
- **Payload**:
  ```json
  {
    "key": "example",
    "value": "test value"
  }
  ```
- **Response**:
  ```json
  {
    "json_data": "{\"example\": \"test value\"}"
  }
  ```

**Example CURL**:
```bash
curl -X POST http://127.0.0.1:5000/json \
-H "Content-Type: application/json" \
-d '{"key": "example", "value": "test value"}'
```

---

### **3. `/string`**

Find matches for a regex pattern in a given string.

- **Method**: `POST`
- **Payload**:
  ```json
  {
    "text": "hello world, hello Flask",
    "pattern": "hello"
  }
  ```
- **Response**:
  ```json
  {
    "matches": ["hello", "hello"]
  }
  ```

**Example CURL**:
```bash
curl -X POST http://127.0.0.1:5000/string \
-H "Content-Type: application/json" \
-d '{"text": "hello world, hello Flask", "pattern": "hello"}'
```

---

### **4. `/compress`**

Compress a text string into Gzip format.

- **Method**: `POST`
- **Payload**:
  ```json
  {
    "text": "This is some text to be compressed"
  }
  ```
- **Response**: Binary Gzip data.

**Example CURL**:
```bash
curl -X POST http://127.0.0.1:5000/compress \
-H "Content-Type: application/json" \
-d '{"text": "This is some text to be compressed"}' --output compressed_data.gz
```

---

### **5. `/image`**

Generate an image with custom text and return it encoded in Base64.

- **Method**: `POST`
- **Payload**:
  ```json
  {
    "text": "Hello, World!"
  }
  ```
- **Response**:
  ```json
  {
    "image": "<Base64 encoded string>"
  }
  ```

**Example CURL**:
```bash
curl -X POST http://127.0.0.1:5000/image \
-H "Content-Type: application/json" \
-d '{"text": "Hello, World!"}'
```

---

## Middleware

The application tracks the execution time for the request lifecycle using Flask's `before_request` and `after_request` hooks. Timing data is added to the response headers:

- `X-Lambda-Start-Time`: The start time of the request.
- `X-Lambda-End-Time`: The end time of the request.
- `X-Lambda-Duration`: The total duration of the request.
- `X-Endpoint-Start-Time`: The start time of the endpoint.
- `X-Endpoint-End-Time`: The end time of the endpoint.
- `X-Endpoint-Duration`: The duration of the endpoint processing.

---

## Testing

To test the API, you can use:
- [Postman](https://www.postman.com/)
- `curl` (examples provided above)
- Any HTTP client library in your preferred programming language.

---

## Contribution

Contributions are welcome! If you find a bug or have a suggestion, please open an issue or submit a pull request.

---

## License

This project is licensed under the [MIT License](LICENSE).