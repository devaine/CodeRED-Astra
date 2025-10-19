import { createPartFromUri, GoogleGenAI } from "@google/genai"
import 'dotenv/config'

const ai = new GoogleGenAI({ apiKey: ${} })