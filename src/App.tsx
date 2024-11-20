import "./App.css";
import Home from "./pages/Home";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Raves from "./pages/Raves";
import RaveDetail from "./pages/RaveDetail";
import Footer from "./components/Footer";
import Add from "./pages/Add";
import AddRave from "./pages/AddRave";
import EditRave from "./pages/EditRave";
import ArtistDetail from "./pages/ArtistDetail";
import AddArtist from "./pages/AddArtist";
import EditArtist from "./pages/EditArtist";

function App() {
    return (
        <BrowserRouter>
            <div className="main-container">
                {/* <h1>Raves Rating</h1> */}
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/raves" element={<Raves />} />
                    <Route path="/rave/:id" element={<RaveDetail />} />
                    <Route path="/artist/:id" element={<ArtistDetail />} />
                    <Route path="/add" element={<Add />} />
                    <Route path="/add-rave" element={<AddRave />} />
                    <Route path="/add-artist" element={<AddArtist />} />
                    <Route path="/edit-rave/:id" element={<EditRave />} />
                    <Route path="/edit-artist/:id" element={<EditArtist />} />
                </Routes>
            </div>
            <Footer />
        </BrowserRouter>
    );
}

export default App;
