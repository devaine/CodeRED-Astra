import { createPartFromUri, GoogleGenAI } from "@google/genai"
import 'dotenv/config'
import fs from "fs"

const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY })

async function uploadLocalPDFs() {
    var pdfList = fs.readdirSync("public/pdfs")

    // Upload each file in /public
    pdfList.forEach(async (path) => {
        console.log("file names: " + path)
        console.log("file names: " + path.slice(0, path.length - 4))

        console.log("UPLOADING")
        const file = await ai.files.upload({
            file: "public/pdfs/" + path,
            config: {
                displayName: path.slice(0, path.length - 4)
            }
        })

        console.log("FETCHING: public/pdfs/" + path)

        // Wait for the file to be processed
        let getFile = await ai.files.get({
            name: file.name
        })

        while (getFile.state === "PROCESSING") {
            let getFile = await ai.files.get({
                name: file.name
            })
            console.log(`Current file status: ${getFile.state}`)
            console.log("File is currently processing, retrying in 5 seconds")

            await new Promise((resolve) => {
                setTimeout(resolve, 5000) // Checks every 5 seconds
            })

            // Error handling
            if (getFile.state === "FAILED") {
                throw new Error("File has failed to process!")
            }
            return file
        }
    })
}

async function main() {
    const prompts = [
        "If possible, using Gemini's Javascript API, how would you grab an image from a PDF sent to the API?"
    ]

    const response = await ai.models.generateContent({
        model: "gemini-2.5-flash",
        contents: prompts
    })

    console.log(response.text)
}

uploadLocalPDFs()
main()

