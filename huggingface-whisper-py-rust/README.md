# Hugging Face example in Rust

Demo to call Python from Rust to run Whisper.

Approach: 
Linking Python with your Rust binary. For the models to respond as quickly as possible they are loaded at startup in video memory (if present). Subsequently, the Rust code invokes a Python function responsible for inference.

Taken from 
https://peterprototypes.com/blog/huggingface-from-rust/
## Pre-req

### Install Python

#### MacOS

```
brew install python@3.13
```

### Init the virtual env for Python:

```
source .venv/bin/activate
```

### Install dependencies:
```
pip install -r requirements.txt
```

### Upgrade:

```
pip install --upgrade pip
```
