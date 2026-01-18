import { useState, useEffect } from 'react'
import { getEngineVersion, pingEngine, VersionInfo } from '../wasm/loader'
import './EngineStatus.css'

function EngineStatus() {
  const [version, setVersion] = useState<VersionInfo | null>(null)
  const [pingResponse, setPingResponse] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    // Load version on mount
    getEngineVersion()
      .then(setVersion)
      .catch((err) => {
        console.error('Failed to get version:', err)
      })
  }, [])

  const handlePing = async () => {
    setLoading(true)
    setPingResponse(null)

    try {
      const response = await pingEngine()
      setPingResponse(`Status: ${response.status}`)
    } catch (err) {
      setPingResponse(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="engine-status">
      <section className="version-section">
        <h2>Engine Version</h2>
        {version ? (
          <div className="version-info">
            <div className="version-row">
              <span className="label">Engine Version:</span>
              <span className="value">{version.engine_version}</span>
            </div>
            <div className="version-row">
              <span className="label">State Version:</span>
              <span className="value">{version.state_version}</span>
            </div>
            <div className="version-row">
              <span className="label">Ruleset ID:</span>
              <span className="value">{version.ruleset_id}</span>
            </div>
          </div>
        ) : (
          <p>Loading version...</p>
        )}
      </section>

      <section className="ping-section">
        <h2>Engine Ping</h2>
        <button
          onClick={handlePing}
          disabled={loading}
          className="ping-button"
        >
          {loading ? 'Pinging...' : 'Ping Engine'}
        </button>
        {pingResponse && (
          <div className="ping-response">
            {pingResponse}
          </div>
        )}
        <p className="hint">
          Check the browser console for Rust logs (dev mode)
        </p>
      </section>
    </div>
  )
}

export default EngineStatus
