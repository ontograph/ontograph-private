# OpenAI Ontocode Python SDK (Beta)

Build Python applications that start Ontocode threads, run turns, stream progress,
and control workspace access.

## Install

Install the SDK. The package name remains `openai-codex` in this stage for compatibility:

```bash
pip install openai-codex
```

## Quickstart

The SDK reuses your existing Ontocode authentication when one is already
available:

```python
from openai_codex import Ontocode

with Ontocode() as codex:
    thread = codex.thread_start()
    result = thread.run("Explain this repository in three bullets.")
    print(result.final_response)
```

`thread.run(...)` returns a `TurnResult` containing the final response,
collected items, and token usage.

## Authentication

Existing Ontocode authentication is reused automatically. To start ChatGPT
browser login explicitly:

```python
from openai_codex import Ontocode

with Ontocode() as codex:
    login = codex.login_chatgpt()
    print(login.auth_url)
    print(login.wait().success)
```

For device-code login:

```python
with Ontocode() as codex:
    login = codex.login_chatgpt_device_code()
    print(login.verification_url, login.user_code)
    login.wait()
```

For API-key login:

```python
with Ontocode() as codex:
    codex.login_api_key("sk-...")
```

## Built-In Help

Use Python's standard `help(openai_codex)`, `help(Ontocode)`, or
`python -m pydoc openai_codex` documentation tools.

## Documentation

- [Getting started](https://github.com/openai/codex/blob/main/sdk/python/docs/getting-started.md)
- [API reference](https://github.com/openai/codex/blob/main/sdk/python/docs/api-reference.md)
- [FAQ](https://github.com/openai/codex/blob/main/sdk/python/docs/faq.md)
- [Examples](https://github.com/openai/codex/blob/main/sdk/python/examples/README.md)

The package is licensed under the
[repository Apache License 2.0](https://github.com/openai/codex/blob/main/LICENSE).
