import { useState } from "react";
import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { marked } from "marked";
import "./App.css";

function Home() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <Link to="/editor">
        <button>Go to Editor</button>
      </Link>
    </main>
  );
}

function Editor() {
  const [text, setText] = useState("");

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Tab") {
      // Allow default behavior if Option (Alt) key is pressed
      if (e.altKey) {
        return;
      }
      
      e.preventDefault();
      const textarea = e.currentTarget;
      const { selectionStart, selectionEnd } = textarea;
      
      // Insert tab character at cursor position
      const newValue = text.substring(0, selectionStart) + "    " + text.substring(selectionEnd);
      
      // Update the text state
      setText(newValue);
      
      // Immediately set the new cursor position on the textarea element
      textarea.value = newValue;
      textarea.selectionStart = textarea.selectionEnd = selectionStart + 4;
    }
  };

  return (
    <main className="container" style={{ display: "flex", flexDirection: "row", height: "100vh" }}>
      <div style={{ flex: 1, padding: "10px", boxSizing: "border-box" }}>
        <h1>Editor</h1>
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Start typing..."
          aria-label="Markdown editor. Use Tab to indent. Use any Option key with Tab to navigate."
          style={{
            width: "100%",
            height: "calc(100% - 40px)",
            padding: "10px",
            fontSize: "16px",
            boxSizing: "border-box",
            resize: "none"
          }}
        />
      </div>
      <div 
        style={{ flex: 1, padding: "10px", borderLeft: "1px solid #ccc", boxSizing: "border-box" }}
        role="region"
        aria-label="Markdown preview"  
      >
        <h1>Markdown Preview</h1>
        <div
          dangerouslySetInnerHTML={{ __html: marked(text) }}
          style={{ overflowY: "auto", height: "calc(100% - 40px)", textAlign: "left", padding: "10px" }}
        />
      </div>
      <Link to="/" style={{ position: "absolute", bottom: "10px", right: "10px" }}>
        <button>Back to Home</button>
      </Link>
    </main>
  );
}

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/editor" element={<Editor />} />
      </Routes>
    </Router>
  );
}

export default App;
