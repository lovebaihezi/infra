import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import Home from "./components/Home";

const App = () => (
    <BrowserRouter>
        <Routes>
            <Route path="/" element={<></>} />
            <Route index element={<Home />} />
        </Routes>
    </BrowserRouter>
);

export default App;
