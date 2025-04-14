import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [keyPresssedMessage, setKeyPresssedMessage] = useState<string | null>(null);

  function listenKeyPressed(payload: string) {
    setKeyPresssedMessage(payload);
  }

  useEffect(() => {
    // キー入力イベントをリッスン
    const unlisten = listen<string>('key-pressed', (event) => {
      listenKeyPressed(event.payload);
    })

    // コンポーネントのアンマウント時にリスナーを解除
    return () => {
      unlisten.then((fn: () => void) => fn())
    }
  }, [])

  return (
    <main className="container">
      {keyPresssedMessage && <p>{keyPresssedMessage}</p>}
    </main>
  );
}

export default App;
