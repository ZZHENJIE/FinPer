import { useState } from "react";
import { FPFetch } from './fetch'
import "./App.css";
import { Button } from "@mui/material";

function App() {
  const [screener, setScreener] = useState("");
  const [stock, setStock] = useState("");

  async function greet() {
    const url_1 = "https://elite.finviz.com/export.ashx?v=111&f=fa_div_pos,sec_technology&auth=badafdcf-0c37-4f48-a476-54b471df6c2c";
    const url_2 = "https://api.finviz.com/api/quote.ashx?aftermarket=true&barsCount=291&dateTo=1724418000&events=true&financialAttachments=&instrument=stock&patterns=false&premarket=true&rev=1740088326885&ticker=WOK&timeframe=d";

    const cvs = await FPFetch(url_1, {
      method: 'GET'
    });

    const object = await FPFetch(url_2, {
      method: 'GET'
    });

    setScreener(cvs.body);
    setStock(object.body);
  }

  return (
    <div className="container">
      <Button variant="contained" onClick={greet}>Hello world</Button>
      <div className="textarea-container">
        <textarea className="data-textarea" value={stock} readOnly placeholder="Stock data will appear here..." />
        <textarea className="data-textarea" value={screener} readOnly placeholder="Screener data will appear here..." />
      </div>
    </div>
  );
}

export default App;