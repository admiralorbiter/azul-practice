import initWasmModule, { 
  get_version, 
  ping,
  list_legal_actions,
  apply_action
} from '../wasm/pkg/engine'

let initialized = false

export interface VersionInfo {
  engine_version: string
  state_version: number
  ruleset_id: string
}

export interface PingResponse {
  status: string
}

/**
 * Initialize the WASM module
 */
export async function init(): Promise<void> {
  if (initialized) {
    return
  }

  await initWasmModule()
  initialized = true
}

/**
 * Get engine version information
 */
export async function getEngineVersion(): Promise<VersionInfo> {
  await init()
  const jsonStr = get_version()
  return JSON.parse(jsonStr) as VersionInfo
}

/**
 * Ping the engine
 */
export async function pingEngine(): Promise<PingResponse> {
  await init()
  const jsonStr = ping()
  return JSON.parse(jsonStr) as PingResponse
}

// Re-export WASM functions
export { list_legal_actions, apply_action }
