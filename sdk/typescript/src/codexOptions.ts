export type OntocodeConfigValue =
  | string
  | number
  | boolean
  | OntocodeConfigValue[]
  | OntocodeConfigObject;

export type OntocodeConfigObject = { [key: string]: OntocodeConfigValue };

export type OntocodeOptions = {
  codexPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the Ontocode CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: OntocodeConfigObject;
  /**
   * Environment variables passed to the Ontocode CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};

export type CodexConfigValue = OntocodeConfigValue;
export type CodexConfigObject = OntocodeConfigObject;
export type CodexOptions = OntocodeOptions;
