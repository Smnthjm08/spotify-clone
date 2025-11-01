import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { ThemeProvider } from "./components/providers/theme-providers";
import { Toaster } from "@smartgrow/ui/components/ui/sonner";
import App from "./App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <App />
      <Toaster position="top-center" />
    </ThemeProvider>
  </StrictMode>,
);
