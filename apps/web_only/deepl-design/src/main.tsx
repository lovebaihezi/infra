import { CssBaseline } from "@mui/material";
import React from "react";
import ReactDOM from "react-dom/client";
import { RecoilRoot } from "recoil";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <CssBaseline />
        <RecoilRoot>
            <App />
        </RecoilRoot>
    </React.StrictMode>
);
