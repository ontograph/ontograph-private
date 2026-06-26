import sys

def main():
    with open("ontocode-rs/core/src/tools/handlers/agent_jobs.rs", "r") as f:
        content = f.read()

    new_content = content.replace("pub(crate) async fn recover_running_items(", "async fn recover_running_items(")
    with open("ontocode-rs/core/src/tools/handlers/agent_jobs.rs", "w") as f:
        f.write(new_content)

if __name__ == "__main__":
    main()
