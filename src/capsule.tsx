import React from "react";
import ReactDOM from "react-dom/client";
import Capsule from "./components/capsule/Capsule";
import "./styles/global.css";

// Capsule window: only renders the floating capsule overlay
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Capsule />
  </React.StrictMode>,
);
