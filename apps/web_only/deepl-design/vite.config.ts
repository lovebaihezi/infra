import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => {
    return {
        host: true,
        plugins: [react()],
    };
});
