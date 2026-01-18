import { useState, useEffect } from 'react'
import { PracticeScreen } from './components/PracticeScreen'
import './App.css'

function App() {
  const [wasmReady, setWasmReady] = useState(false)
  const [wasmError, setWasmError] = useState<string | null>(null)

  useEffect(() => {
    // Initialize WASM
    import('./wasm/loader').then((loader) => {
      loader.init().then(() => {
        setWasmReady(true)
      }).catch((err) => {
        setWasmError(err.message || 'Failed to initialize WASM')
      })
    }).catch((err) => {
      setWasmError(err.message || 'Failed to load WASM module')
    })
  }, [])

  return (
    <div className="app">
      <header className="app-header">
        <h1>Azul Practice Tool</h1>
        <p>Sprint 02 - Interactive Board</p>
      </header>

      <main className="app-main">
        {wasmError ? (
          <div className="error">
            <p>Error: {wasmError}</p>
            <p className="hint">
              Make sure to build WASM first: <code>pnpm wasm:build</code>
            </p>
          </div>
        ) : !wasmReady ? (
          <div className="loading">
            <p>Loading WASM module...</p>
          </div>
        ) : (
          <PracticeScreen />
        )}
      </main>
    </div>
  )
}

export default App
