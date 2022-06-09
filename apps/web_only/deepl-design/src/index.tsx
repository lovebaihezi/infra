import { CssBaseline, StyledEngineProvider } from "@mui/material";
import React from "react";
import ReactDOM from "react-dom/client";
import { RecoilRoot } from "recoil";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <CssBaseline />
        <StyledEngineProvider injectFirst>
            <RecoilRoot>
                <App />
            </RecoilRoot>
        </StyledEngineProvider>
    </React.StrictMode>
);
