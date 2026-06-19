use ontocode_arg0::Arg0DispatchPaths;
use ontocode_arg0::arg0_dispatch_or_else;
use ontocode_mcp_server::run_main;
use ontocode_utils_cli::CliConfigOverrides;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        run_main(
            arg0_paths,
            CliConfigOverrides::default(),
            /*strict_config*/ false,
        )
        .await?;
        Ok(())
    })
}
