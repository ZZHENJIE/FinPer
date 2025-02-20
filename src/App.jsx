import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [screener, setScreener] = useState("");
  const [stock, setStock] = useState("");

  async function greet() {
    const screener_url = "https://elite.finviz.com/export.ashx?v=111&f=fa_div_pos,sec_technology&auth=badafdcf-0c37-4f48-a476-54b471df6c2c";
    const stock_url = "https://api.finviz.com/api/quote.ashx?aftermarket=true&barsCount=291&dateTo=1724418000&events=true&financialAttachments=&instrument=stock&patterns=false&premarket=true&rev=1740088326885&ticker=WOK&timeframe=d";

    const cvs = await invoke("fetch", { url: screener_url });
    const json = await invoke("fetch", { url: stock_url });

    setScreener(cvs);
    setStock(json);
  }

  return (
    <div className="container">
      <button className="fetch-button" onClick={greet}>
        Fetch Data
      </button>
      <div className="textarea-container">
        <textarea className="data-textarea" value={stock} readOnly placeholder="Stock data will appear here..." />
        <textarea className="data-textarea" value={screener} readOnly placeholder="Screener data will appear here..." />
      </div>
    </div>
  );
}

export default App;