import express from "express";
import bodyParser from "body-parser";
import axios from "axios";
import multer from "multer";
import path from "path";
import { fileURLToPath } from 'url';
import fs from "fs";

const app = new express();
const port = 3000;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

app.use(express.static('public'));
app.use('/uploads', express.static('uploads'));
app.use(bodyParser.urlencoded({extended: true}));
app.use(bodyParser.json());

const storage = multer.diskStorage({
  destination: function (req, file, cb) {
    cb(null, path.join(__dirname,'/uploads'));
  },
  filename: function (req, file, cb) {
    cb(null, file.originalname);
  }
})
const upload = multer({ storage: storage })



//Render the main page
app.get("/", async (req, res) => {
    try{
        const response = await axios.get(`${API_URL}/all`);
        res.render("file", { data: response.data });
    }catch(error){
        console.error(error);
        res.status(500).json("Error fetching items");
    }
})

app.post("/upload", upload.single('image'), async (req, res) => {
    const data = {
        ...req.body,
        fileName: req.file.originalname,
        path: req.file.path
    }
    try{
        await axios.post(`${API_URL}/add`, data);
        res.redirect("/");
    }catch(error){
        console.error(error);
        res.status(500).json("Error uploading item");
    }
})

app.listen(port, () => {
    console.log("API is listening on port " + port);
})