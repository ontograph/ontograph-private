import { OntocodeOptions } from "./codexOptions";
import { OntocodeExec } from "./exec";
import { Thread } from "./thread";
import { ThreadOptions } from "./threadOptions";

/**
 * Ontocode is the main class for interacting with the Ontocode agent.
 *
 * Use the `startThread()` method to start a new thread or `resumeThread()` to resume a previously started thread.
 */
export class Ontocode {
  private exec: OntocodeExec;
  private options: OntocodeOptions;

  constructor(options: OntocodeOptions = {}) {
    const { codexPathOverride, env, config } = options;
    this.exec = new OntocodeExec(codexPathOverride, env, config);
    this.options = options;
  }

  /**
   * Starts a new conversation with an agent.
   * @returns A new thread instance.
   */
  startThread(options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options);
  }

  /**
   * Resumes a conversation with an agent based on the thread id.
   * Threads are persisted in ~/.codex/sessions.
   *
   * @param id The id of the thread to resume.
   * @returns A new thread instance.
   */
  resumeThread(id: string, options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options, id);
  }
}

export { Ontocode as Codex };
