import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import jsconfigPaths from "vite-jsconfig-paths";
import tailwindcss from "@tailwindcss/vite";

try {
	process.loadEnvFile(".env")
} catch (error) {
	console.log("Env file not found!\n" + error)
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [tailwindcss(), react(), jsconfigPaths()],
  resolve: {
    alias: {
      src: "/src",
    },
  },
  // Defines envrionmental files across all src code b/c prefix is usually "VITE"
	define: {
		'import.meta.env.GEMINI_API_KEY': JSON.stringify(process.env.GEMINI_API_KEY),
	}
});
